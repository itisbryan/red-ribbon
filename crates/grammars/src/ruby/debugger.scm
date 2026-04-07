; Ruby debug variables and scopes for rdbg integration

; Local variables
(assignment
  left: (identifier) @debug-variable)

; Method parameters
(method_parameters
  (identifier) @debug-variable)

; Block parameters
(block_parameters
  (identifier) @debug-variable)

; Splat parameters
(splat_parameter
  name: (identifier) @debug-variable)

; Hash splat parameters
(hash_splat_parameter
  name: (identifier) @debug-variable)

; Optional keyword parameters
(optional_keyword_parameter
  name: (identifier) @debug-variable)

; Keyword parameters
(keyword_parameter
  name: (identifier) @debug-variable)

; Instance variables
(instance_variable
  name: (identifier) @debug-variable)

; Class variables
(class_variable
  name: (identifier) @debug-variable)

; Global variables
(global_variable
  name: (identifier) @debug-variable)

; Constants
(constant
  name: (identifier) @debug-variable)

; Blocks and methods as debug scopes
(method
  body: (_) @debug-scope)

(do_block
  body: (_) @debug-scope)

(block
  body: (_) @debug-scope)

(class
  body: (_) @debug-scope)

(module
  body: (_) @debug-scope)

; Iterator variables
(for
  iterator: (identifier) @debug-variable)

; Rescue exception variables
(rescue
  exception: (exception_variable
    name: (identifier) @debug-variable))
