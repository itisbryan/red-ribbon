; Ruby outline — modules, classes, methods, attr_*, Rails macros

; Modules
(module
  name: (constant) @name
  (#set! icon "package")) @item

; Classes
(class
  name: (constant) @name
  (#set! icon "struct")) @item

; Singleton classes
(singleton_class
  name: (constant) @name
  (#set! icon "struct")) @item

; Method definitions
(method
  name: (identifier) @name
  (#set! icon "method")) @item

; Singleton methods
(singleton_method
  name: (identifier) @name
  (#set! icon "method")) @item

; initialize
(method
  name: (identifier) @name
  (#eq? @name "initialize")
  (#set! icon "constructor")) @item

; attr_reader / attr_writer / attr_accessor / attr
(call
  method: (identifier) @_method
  (#match? @_method "^attr_")
  arguments: (argument_list
    (symbol) @name)) @item

; ActiveRecord associations
(call
  method: (identifier) @_method
  (#any-of? @_method "has_many" "belongs_to" "has_one" "has_and_belongs_to_many")
  arguments: (argument_list
    (symbol) @name)) @item

; ActiveRecord validations
(call
  method: (identifier) @_method
  (#any-of? @_method "validates" "validate" "validates_presence_of" "validates_uniqueness_of")
  arguments: (argument_list
    [
      (symbol) @name
      (string_content) @name
    ])) @item

; ActiveRecord scopes
(call
  method: (identifier) @_method
  (#eq? @_method "scope")
  arguments: (argument_list
    (symbol) @name)) @item

; Rails callbacks
(call
  method: (identifier) @_method
  (#match? @_method "^(before_|after_|around_)(action|filter|validate|save|create|update|destroy|commit|rollback)")
  arguments: (argument_list
    (symbol) @name)) @item

; Public, private, protected access modifiers
(call
  method: (identifier) @_method
  (#any-of? @_method "public" "private" "protected")
  arguments: (argument_list
    (symbol) @name)) @item

; Constants (CONSTANT_CASE)
(assignment
  left: (constant
    name: (identifier) @name)) @item

; class methods using self.method_name
(singleton_method
  target: (self)
  name: (identifier) @name
  (#set! icon "method.static")) @item
