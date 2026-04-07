; Ruby injections — embedded languages inside Ruby

; ERB is handled by the separate ERB language, not here
; Comments use Ruby
([
  (comment)
] @injection.content
  (#set! injection.language "comment"))

; SQL inside string literals that look like SQL
(call
  method: (identifier) @_method
  (#any-of? @_method "execute" "find_by_sql" "select_all" "select_one" "select_values" "select_value")
  arguments: (argument_list
    [
      (string (string_content) @injection.content)
      (bare_string (string_content) @injection.content)
    ])
  (#set! injection.language "sql"))

; Regex inside %r{} or /.../
(regex) @injection.content
(#set! injection.language "regex")

; HTML inside string literals that look like they contain HTML (Rails render calls)
(call
  method: (identifier) @_method
  (#any-of? @_method "render")
  arguments: (argument_list
    (hash
      (pair
        key: (symbol) @_key
        (#eq? @_key "html:")
        value: [
          (string (string_content) @injection.content)
          (bare_string (string_content) @injection.content)
        ])))
  (#set! injection.language "html"))

; Shell commands inside backtick strings and %x{} strings
(heredoc_string
  (heredoc_begin) @_begin
  (#match? @_begin "`")) @injection.content
(#set! injection.language "bash")
