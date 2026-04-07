; Ruby runnables — RSpec, Minitest, Rails tests, and Cucumber

; === RSpec ===

; RSpec describe/class/context blocks (top-level)
(call
  method: (identifier) @_method
  (#any-of? @_method "describe" "context" "feature")
  arguments: (argument_list
    [
      (string (string_content) @run)
      (constant) @run
      (string_content) @run
    ])) @_ruby-rspec-group
(#set! tag ruby-rspec-group)

; RSpec it/specify/scenario blocks
(call
  method: (identifier) @_method
  (#any-of? @_method "it" "specify" "scenario" "example")
  arguments: (argument_list
    [
      (string (string_content) @run)
      (string_content) @run
    ])) @_ruby-rspec-test
(#set! tag ruby-rspec-test)

; === Minitest ===

; Minitest test "name" do blocks
(call
  method: (identifier) @_method
  (#eq? @_method "test")
  arguments: (argument_list
    (string (string_content) @run)) @_ruby-minitest-test
(#set! tag ruby-minitest-string-test)

; Minitest def test_* methods
(method
  name: (identifier) @run @_test_name
  (#match? @_test_name "^test_")) @_ruby-minitest-method-test
(#set! tag ruby-minitest-method-test)

; Minitest ActiveSupport test blocks (test "something" do)
(call
  method: (identifier) @_method
  (#eq? @_method "test")
  arguments: (argument_list
    (string (string_content) @run))
  block: (do_block)) @_ruby-minitest-block-test
(#set! tag ruby-minitest-block-test)

; === Cucumber ===

; Cucumber feature/scenario blocks
(call
  method: (identifier) @_method
  (#any-of? @_method "Feature" "Scenario" "ScenarioOutline" "scenario" "feature" "scenario_outline")
  arguments: (argument_list
    (string (string_content) @run))) @_ruby-cucumber-test
(#set! tag ruby-cucumber-test)
