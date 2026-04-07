; Highlights for Ruby

; Keywords
[
  "alias"
  "and"
  "begin"
  "break"
  "case"
  "class"
  "def"
  "defined?"
  "do"
  "else"
  "elsif"
  "end"
  "ensure"
  "fail"
  "false"
  "for"
  "if"
  "in"
  "lambda"
  "load"
  "module"
  "next"
  "nil"
  "not"
  "or"
  "raise"
  "redo"
  "rescue"
  "require"
  "return"
  "self"
  "send"
  "then"
  "true"
  "undef"
  "unless"
  "until"
  "when"
  "while"
  "yield"
] @keyword

; Control keywords (value-like)
[
  "__FILE__"
  "__LINE__"
  "__dir__"
  "__method__"
  "__callee__"
  "__caller__"
] @variable.builtin

; Constants
[
  "true"
  "false"
  "nil"
  "self"
] @constant.builtin

; Comments
(comment) @comment

; Strings
[
  (string_content)
  (bare_string)
  (string_array)
  (heredoc_string)
  (heredoc_begin)
  (heredoc_body)
  (heredoc_end)
  (symbol_literal)
] @string

; Escape sequences in strings
(escape) @string.escape

; Interpolation inside strings
(interpolation
  "#" @punctuation.special
) @embedded

; Numbers
(integer) @constant.numeric
(float) @constant.numeric
(rational) @constant.numeric
(complex) @constant.numeric

; Symbols
(symbol) @string.special.symbol

; Regular expressions
(regex) @string.regexp

; Operators
[
  "="
  "=>"
  "->"
  "+@"
  "-"
  "+"
  "*"
  "/"
  "%"
  "**"
  "!"
  "~"
  "&"
  "|"
  "^"
  "<"
  ">"
  "<="
  ">="
  "=="
  "!="
  "&&"
  "||"
  "<<"
  ">>"
  ".."
  "..."
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  ","
  "."
  ";"
  "::"
] @punctuation.bracket

; Delimiters
[
  ";"
  "\n"
] @punctuation.delimiter

; Instance variables
(identifier) @variable

; Instance variables starting with @ (instance vars)
(identifier) @variable.builtin
(#match? @variable.builtin "^@")

; Global variables ($...)
(global_variable
  name: (identifier) @variable.builtin)

; Class variables (@@...)
(class_variable
  name: (identifier) @variable.builtin)

; Constants / CONSTANT_CASE
(constant
  name: (identifier) @constant)

; Self
(self) @variable.special

; Super
(super) @variable.special

; Method definitions
(method
  name: (identifier) @function.method
  parameters: (method_parameters)? @parameter
  body: (_)? @function.method.inner) @function.method

; Singleton method definitions
(singleton_method
  name: (identifier) @function.method
  parameters: (method_parameters)? @parameter) @function.method

; Class/module definitions
(class
  name: (constant) @type
  body: (_)? @type.inner) @type

(module
  name: (constant) @type
  body: (_)? @type.inner) @type

; Method calls
(call
  method: (identifier) @function.call) @function.call

; Block calls (do..end)
(do_block
  "do" @keyword
  body: (_)? @function.call) @function.call

; Block calls (brace blocks)
(block
  "{" @punctuation.bracket
  body: (_)
  "}" @punctuation.bracket)

; Lambda
(lambda
  parameters: (block_parameters)? @parameter
  body: (_)? @function)

; Assignment targets
(assignment
  left: (identifier) @variable
  right: (_)?)

; Operator assignment
(operator_assignment
  left: (identifier) @variable
  operator: (_)? @operator
  right: (_)?)

; if/elsif/else/unless
[
  (if
    "if" @keyword
    condition: (_)? @conditional
    "then" @keyword
    body: (_)?)
  (elsif
    "elsif" @keyword
    condition: (_)? @conditional)
  (else
    "else" @keyword)
  (unless
    "unless" @keyword
    condition: (_)? @conditional
    "then" @keyword)
] @conditional

; case/when
(case
  "case" @keyword
  value: (_)? @conditional
  "when" @keyword
  body: (_)?)

; while/until
[
  (while
    "while" @keyword
    condition: (_)? @conditional
    "do" @keyword)
  (until
    "until" @keyword
    condition: (_)? @conditional
    "do" @keyword)
] @repeat

; for loop
(for
  "for" @keyword
  iterator: (identifier) @variable
  "in" @keyword
  body: (_)?)

; begin/rescue/ensure
[
  (begin
    "begin" @keyword)
  (rescue
    "rescue" @keyword)
  (ensure
    "ensure" @keyword)
] @keyword

; return
(return
  "return" @keyword
  value: (_)?)

; yield
(yield
  "yield" @keyword)

; raise
(raise
  "raise" @keyword)

; require
(require
  "require" @keyword)

; include / extend / prepend
[
  (include
    "include" @keyword)
  (extend
    "extend" @keyword)
  (prepend
    "prepend" @keyword)
] @keyword

; attr_reader / attr_writer / attr_accessor
(call
  method: (identifier) @function.macro
  (#match? @function.macro "^attr_"))

; ActiveRecord macros
(call
  method: (identifier) @function.macro
  (#match? @function.macro "^(has_many|belongs_to|has_one|has_and_belongs_to_many|validates|scope|before_action|after_action|before_filter|after_filter|before_validate|after_validate|render|redirect_to|respond_to|resource|resources|namespace|root|get|post|put|patch|delete|head|link_to|form_with|form_for|button_to|content_tag|csrf_meta_tags|javascript_include_tag|stylesheet_link_tag)$"))

; Rails-specific class definitions detected by inheritance
(class
  name: (constant) @type
  superclass: (argument_list
    (constant) @type.inherited
    (#match? @type.inherited "^(ApplicationController|ApplicationRecord|ApplicationJob|ActiveRecord::Base|ActiveRecord::Migration|ActionMailer::Base|ActionDispatch::IntegrationTest|ActionController::API|ActionController::Base|ActionView::Helpers::FormHelper)$")))

; ERB output tags
(erb_content) @embedded

; Heredoc
(string_array
  "<<" @punctuation.delimiter
  (heredoc_body) @string
  (heredoc_end) @string.special)

; Numbered parameters
(numbered_parameter
  scope: (_)? @variable.builtin)
