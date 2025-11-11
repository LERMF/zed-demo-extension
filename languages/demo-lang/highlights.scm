; Tree-sitter query file for syntax highlighting
; Example highlights for demo language

; Keywords
[("if") ("else") ("while") ("for") ("return")] @keyword

; Functions
(function_declaration
  name: (identifier) @function)

; Comments
(comment) @comment

; Strings
(string_literal) @string

; Numbers
(number_literal) @number

; Operators
["=" "+" "-" "*" "/" "=="] @operator