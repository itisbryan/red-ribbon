; Ruby bracket matching

("(" @open
  ")" @close)

("[" @open
  "]" @close)

("{" @open
  "}" @close)

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

(("'" @open
  "'" @close)
  (#set! rainbow.exclude))

(("|" @open
  "|" @close)
  (#set! rainbow.exclude))

; do..end as matching pair
(do_block
  "do" @open
  "end" @close)

; if..end
(if
  "if" @open
  "end" @close)

(unless
  "unless" @open
  "end" @close)

; class..end
(class
  "class" @open
  "end" @close)

; module..end
(module
  "module" @open
  "end" @close)

; def..end
(method
  "def" @open
  "end" @close)

; begin..end
(begin
  "begin" @open
  "end" @close)

; case..end
(case
  "case" @open
  "end" @close)

; while..end / until..end / for..end
(while
  "while" @open
  "end" @close)

(until
  "until" @open
  "end" @close)

(for
  "for" @open
  "end" @close)
