; Ruby indentation rules

; Indent inside do..end blocks
(do_block
  body: (_)) @indent

; Indent inside def..end
(method
  body: (_)) @indent

; Indent inside class..end
(class
  body: (_)) @indent

; Indent inside module..end
(module
  body: (_)) @indent

; Indent inside if/elsif/else/unless
[
  (if
    "then" @end
    body: (_))
  (elsif
    "then" @end
    body: (_))
  (else
    body: (_))
  (unless
    "then" @end
    body: (_))
] @indent

; Indent inside while/until/for
[
  (while
    "do" @end
    body: (_))
  (until
    "do" @end
    body: (_))
  (for
    "do" @end
    body: (_))
] @indent

; Indent inside begin/rescue/ensure
[
  (begin
    body: (_))
  (rescue
    body: (_))
  (ensure
    body: (_))
] @indent

; Indent inside case/when
(case
  body: (_)) @indent

(when
  body: (_)) @indent

; Indent inside brackets
(_
  "{"
  "}" @end) @indent

(_
  "("
  ")" @end) @indent

(_
  "["
  "]" @end) @indent

; Multi-line call chains
(call
  block: (_)) @indent

; Array/hash literals spanning multiple lines
(array
  "[" @open
  "]" @close) @indent

(hash
  "{" @open
  "}" @close) @indent

; Lambda bodies
(lambda
  body: (_)) @indent

; Brace blocks
(block
  "{" @open
  body: (_)
  "}" @close) @indent
