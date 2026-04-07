; Ruby text objects for vim-like navigation

; Functions (def..end)
(method
  body: (_
    (_)* @function.inside)) @function.around

(singleton_method
  body: (_
    (_)* @function.inside)) @function.around

; Classes
(class
  body: (_
    (_)* @class.inside)) @class.around

; Modules
(module
  body: (_
    (_)* @class.inside)) @class.around

; Blocks (do..end)
(do_block
  body: (_
    (_)* @function.inside)) @function.around

; Brace blocks
(block
  body: (_
    (_)* @function.inside)) @function.around

; Comments
(comment) @comment.around

; If/unless blocks
(if
  "if" @_start
  body: (_)* @conditional.inside
  "end" @_end) @conditional.around

(unless
  "unless" @_start
  body: (_)* @conditional.inside
  "end" @_end) @conditional.around

; case/when
(case
  body: (_)* @conditional.inside) @conditional.around

; Begin/rescue/ensure
(begin
  body: (_)* @block.inside) @block.around
