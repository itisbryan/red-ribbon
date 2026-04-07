use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
pub use language::*;
use language::{ContextLocation, ContextProvider};
use language::{Buffer, LanguageName, ManifestName, ManifestProvider, ManifestQuery};
use language::{Toolchain, ToolchainList, ToolchainLister, ToolchainMetadata};
use lsp::{LanguageServerBinary, LanguageServerName};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::SemanticTokenRules;
use smol::fs as async_fs;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::rel_path::RelPath;

pub(crate) fn semantic_token_rules() -> SemanticTokenRules {
    let content = grammars::get_file("ruby/semantic_token_rules.json")
        .expect("missing ruby/semantic_token_rules.json");
    let json = std::str::from_utf8(&content.data).expect("invalid utf-8 in semantic_token_rules");
    settings::parse_json_with_comments::<SemanticTokenRules>(json)
        .expect("failed to parse ruby semantic_token_rules.json")
}

// ── LSP adapter (ruby-lsp) ─────────────────────────────────────────────────

pub struct RubyLspAdapter;

impl RubyLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("ruby-lsp");
}

impl LspInstaller for RubyLspAdapter {
    type BinaryVersion = Option<String>;

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
        _pre_release: bool,
        _cx: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        // ruby-lsp is installed as a gem, not fetched from GitHub.
        // Return None to indicate no remote version fetching.
        Ok(None)
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _toolchain: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        // Check for ruby-lsp via `bundle exec ruby-lsp` first (project-local),
        // then fall back to a globally installed gem.
        let path = delegate.which("ruby-lsp".as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            arguments: vec![],
            env: None,
        })
    }

    async fn fetch_server_binary(
        &self,
        _version: Self::BinaryVersion,
        _container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        anyhow::bail!(
            "ruby-lsp is installed as a gem. Run `gem install ruby-lsp` \
             or add it to your Gemfile."
        )
    }

    async fn cached_server_binary(
        &self,
        _container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None
    }
}

#[async_trait(?Send)]
impl super::LspAdapter for RubyLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

// ── Manifest provider (project detection via Gemfile) ─────────────────────

pub(crate) struct GemfileManifestProvider;

impl ManifestProvider for GemfileManifestProvider {
    fn name(&self) -> ManifestName {
        SharedString::new_static("Gemfile").into()
    }

    fn search(
        &self,
        ManifestQuery {
            path,
            depth,
            delegate,
        }: ManifestQuery,
    ) -> Option<Arc<RelPath>> {
        for ancestor in path.ancestors().take(depth) {
            let p = ancestor.join(RelPath::unix("Gemfile").unwrap());
            if delegate.exists(&p, Some(false)) {
                return Some(ancestor.into());
            }
        }

        None
    }
}

// ── Context provider (AI assistant + task variables) ───────────────────────

pub(crate) struct RubyContextProvider;

const RUBY_ACTIVE_TOOLCHAIN_PATH: VariableName =
    VariableName::Custom(Cow::Borrowed("RUBY_ACTIVE_ZED_TOOLCHAIN"));

impl ContextProvider for RubyContextProvider {
    fn build_context(
        &self,
        _variables: &task::TaskVariables,
        _location: ContextLocation<'_>,
        _: Option<HashMap<String, String>>,
        _toolchains: Arc<dyn language::LanguageToolchainStore>,
        _cx: &mut App,
    ) -> Task<Result<task::TaskVariables>> {
        let active_toolchain = String::from("ruby");

        let mut result_vars = task::TaskVariables::default();
        result_vars.insert(RUBY_ACTIVE_TOOLCHAIN_PATH.clone(), active_toolchain);

        Task::ready(Ok(result_vars))
    }

    fn associated_tasks(
        &self,
        _buffer: Option<Entity<Buffer>>,
        _cx: &App,
    ) -> Task<Option<TaskTemplates>> {
        let tasks = vec![
            // Run current file with Ruby
            TaskTemplate {
                label: format!("ruby '{}'", VariableName::File.template_value()),
                command: RUBY_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![VariableName::File.template_value_with_whitespace()],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Execute a selection
            TaskTemplate {
                label: "execute selection".to_owned(),
                command: RUBY_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![
                    "-e".to_owned(),
                    VariableName::SelectedText.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // RSpec: run file
            TaskTemplate {
                label: format!("bundle exec rspec '{}'", VariableName::File.template_value()),
                command: "bundle".to_owned(),
                args: vec![
                    "exec".to_owned(),
                    "rspec".to_owned(),
                    VariableName::File.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // RSpec: run test at line
            TaskTemplate {
                label: format!(
                    "bundle exec rspec {}:{}",
                    VariableName::File.template_value(),
                    VariableName::Row.template_value()
                ),
                command: "bundle".to_owned(),
                args: vec![
                    "exec".to_owned(),
                    "rspec".to_owned(),
                    format!(
                        "{}:{}",
                        VariableName::File.template_value(),
                        VariableName::Row.template_value(),
                    ),
                ],
                tags: vec![
                    "ruby-rspec-test".to_owned(),
                    "ruby-rspec-group".to_owned(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Minitest: run file
            TaskTemplate {
                label: format!(
                    "bundle exec ruby -Itest '{}'",
                    VariableName::File.template_value()
                ),
                command: "bundle".to_owned(),
                args: vec![
                    "exec".to_owned(),
                    "ruby".to_owned(),
                    "-Itest".to_owned(),
                    VariableName::File.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                tags: vec![
                    "ruby-minitest-method-test".to_owned(),
                    "ruby-minitest-string-test".to_owned(),
                    "ruby-minitest-block-test".to_owned(),
                ],
                ..TaskTemplate::default()
            },
            // Minitest: run test at line
            TaskTemplate {
                label: format!(
                    "bundle exec ruby -Itest {}:{}",
                    VariableName::File.template_value(),
                    VariableName::Row.template_value()
                ),
                command: "bundle".to_owned(),
                args: vec![
                    "exec".to_owned(),
                    "ruby".to_owned(),
                    "-Itest".to_owned(),
                    format!(
                        "{}:{}",
                        VariableName::File.template_value(),
                        VariableName::Row.template_value(),
                    ),
                ],
                tags: vec![
                    "ruby-minitest-method-test".to_owned(),
                    "ruby-minitest-string-test".to_owned(),
                    "ruby-minitest-block-test".to_owned(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Rails test: run file
            TaskTemplate {
                label: format!(
                    "bin/rails test '{}'",
                    VariableName::File.template_value()
                ),
                command: "bin/rails".to_owned(),
                args: vec![
                    "test".to_owned(),
                    VariableName::File.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Rails test: run at line
            TaskTemplate {
                label: format!(
                    "bin/rails test {}:{}",
                    VariableName::File.template_value(),
                    VariableName::Row.template_value()
                ),
                command: "bin/rails".to_owned(),
                args: vec![
                    "test".to_owned(),
                    format!(
                        "{}:{}",
                        VariableName::File.template_value(),
                        VariableName::Row.template_value(),
                    ),
                ],
                tags: vec![
                    "ruby-rspec-test".to_owned(),
                    "ruby-rspec-group".to_owned(),
                    "ruby-minitest-method-test".to_owned(),
                    "ruby-minitest-string-test".to_owned(),
                    "ruby-minitest-block-test".to_owned(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Bundle install
            TaskTemplate {
                label: "bundle install".to_owned(),
                command: "bundle".to_owned(),
                args: vec!["install".to_owned()],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Rake task
            TaskTemplate {
                label: format!("rake '{}'", VariableName::Stem.template_value()),
                command: "bundle".to_owned(),
                args: vec![
                    "exec".to_owned(),
                    "rake".to_owned(),
                    VariableName::Stem.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
        ];

        Task::ready(Some(TaskTemplates(tasks)))
    }
}

// ── Toolchain provider (detect Ruby installations) ─────────────────────────

pub(crate) struct RubyToolchainProvider;

impl RubyToolchainProvider {
    pub fn new(_fs: Arc<dyn Fs>) -> Self {
        Self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RubyToolchainData {
    pub path: String,
    pub version: Option<String>,
    pub manager: Option<String>,
}

#[async_trait]
impl ToolchainLister for RubyToolchainProvider {
    async fn list(
        &self,
        worktree_root: PathBuf,
        _subroot_relative_path: Arc<RelPath>,
        _project_env: Option<HashMap<String, String>>,
    ) -> ToolchainList {
        let mut toolchains = Vec::new();

        // Detect Ruby via version managers and system path
        let candidates = detect_ruby_binaries(&worktree_root).await;

        for (path, manager) in candidates {
            let version = get_ruby_version(&path).await;
            let display_version = version.as_deref().unwrap_or("unknown");

            toolchains.push(Toolchain {
                language_name: LanguageName::new_static("Ruby"),
                path: path.to_string_lossy().into_owned().into(),
                name: format!("Ruby {} ({})", display_version, manager).into(),
                as_json: serde_json::to_value(RubyToolchainData {
                    path: path.to_string_lossy().into_owned(),
                    version,
                    manager: Some(manager.to_string()),
                })
                .unwrap_or_default(),
                
            });
        }

        toolchains.dedup();
        ToolchainList {
            toolchains,
            default: None,
            groups: Default::default(),
        }
    }

    fn meta(&self) -> ToolchainMetadata {
        ToolchainMetadata {
            term: SharedString::new_static("Ruby"),
            new_toolchain_placeholder: SharedString::new_static("Path to the ruby executable"),
            manifest_name: ManifestName::from(SharedString::new_static("Gemfile")),
        }
    }

    async fn resolve(
        &self,
        path: PathBuf,
        _env: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Toolchain> {
        let version = get_ruby_version(&path).await;
        let display_version = version.as_deref().unwrap_or("unknown");

        Ok(Toolchain {
            language_name: LanguageName::new_static("Ruby"),
            path: path.to_string_lossy().into_owned().into(),
            name: format!("Ruby {}", display_version).into(),
            as_json: serde_json::to_value(RubyToolchainData {
                path: path.to_string_lossy().into_owned(),
                version,
                manager: None,
            })
            .unwrap_or_default(),
            
        })
    }

    fn activation_script(
        &self,
        _toolchain: &Toolchain,
        _shell: task::ShellKind,
        _cx: &App,
    ) -> futures::future::BoxFuture<'static, Vec<String>> {
        Box::pin(async move { vec![] })
    }
}

async fn detect_ruby_binaries(_worktree_root: &Path) -> Vec<(PathBuf, &'static str)> {
    let mut candidates = Vec::new();

    // Check for rbenv shims
    if let Ok(rbenv_root) = std::env::var("RBENV_ROOT")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.rbenv")))
    {
        let rbenv_shim = PathBuf::from(rbenv_root).join("shims/ruby");
        if rbenv_shim.exists() {
            candidates.push((rbenv_shim, "rbenv"));
        }
    }

    // Check for asdf
    if let Ok(asdf_dir) = std::env::var("ASDF_DIR")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.asdf")))
    {
        let asdf_shim = PathBuf::from(asdf_dir).join("shims/ruby");
        if asdf_shim.exists() {
            candidates.push((asdf_shim, "asdf"));
        }
    }

    // Check for mise
    if let Some(mise_data) = std::env::var("MISE_DATA_DIR")
        .ok()
        .or_else(|| {
            std::env::var("XDG_DATA_HOME")
                .ok()
                .map(|x| format!("{x}/mise"))
                .or_else(|| std::env::var("HOME").ok().map(|h| format!("{h}/.local/share/mise")))
        })
    {
        let installs_dir = PathBuf::from(mise_data).join("installs/ruby");
        if let Ok(mut entries) = async_fs::read_dir(&installs_dir).await {
            while let Some(entry) = entries.next().await {
                if let Ok(entry) = entry {
                    let bin_path = entry.path().join("bin/ruby");
                    if bin_path.exists() {
                        candidates.push((bin_path, "mise"));
                    }
                }
            }
        }
    }

    // Check for chruby
    if let Ok(home) = std::env::var("HOME") {
        let chruby_dir = PathBuf::from(home).join(".rubies");
        if chruby_dir.exists() {
            if let Ok(mut entries) = async_fs::read_dir(&chruby_dir).await {
                while let Some(entry) = entries.next().await {
                    if let Ok(entry) = entry {
                        let bin_path = entry.path().join("bin/ruby");
                        if bin_path.exists() {
                            candidates.push((bin_path, "chruby"));
                        }
                    }
                }
            }
        }
    }

    // Check system ruby as fallback
    if let Ok(output) = smol::process::Command::new("which").arg("ruby").output().await {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                candidates.push((PathBuf::from(path), "system"));
            }
        }
    }

    candidates
}

async fn get_ruby_version(ruby_path: &Path) -> Option<String> {
    let output = smol::process::Command::new(ruby_path)
        .arg("--version")
        .output()
        .await
        .ok()?;

    let version_str = String::from_utf8_lossy(&output.stdout);
    // Parse "ruby 3.3.0 (2023-12-25 revision ...) [arch]" -> "3.3.0"
    version_str
        .split_whitespace()
        .nth(1)
        .map(|v| v.to_string())
}
