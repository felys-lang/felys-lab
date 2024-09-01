# Felys Grammar Future Design

Here are some immature design of Felys. PEG parser does not support left recursion, a modified packrat parser (PEG with a caching model) does. This is exactly what CPython did make the parser express programmers' intention better.

## Program

```
program:
    \ statement* `EOF`
```

## Statement

```
statement:
    \ stmt_if
    \ stmt_while
    \ stmt_for
    \ stmt_loop
    \ stmt_simple
    \ block
    \ assignment `;`
    \ expression `;`
    \ `return` expression? `;`
    \ `break` expression? `;`
    \ `continue` `;`

stmt_if:
    \ `if` unwrap_assign block stmt_elif
    \ `if` unwrap_assign block stmt_else
    \ `if` unwrap_assign block
    \ `if` expression block stmt_elif
    \ `if` expression block stmt_else
    \ `if` expression block

stmt_elif:
    \ `elif` unwrap_assign block stmt_elif
    \ `elif` unwrap_assign block stmt_else
    \ `elif` unwrap_assign block
    \ `elif` expression block stmt_elif
    \ `elif` expression block stmt_else
    \ `elif` expression block

stmt_while:
    \ `while` unwrap_assign block stmt_else
    \ `while` unwrap_assign block
    \ `while` expression block stmt_else
    \ `while` expression block

stmt_for:
    \ `for` name `in` expression block stmt_else
    \ `for` name `in` expression block

stmt_loop:
    \ `loop` block stmt_else
    \ `loop` block

stmt_else:
    \ `else` block

block:
    \ `{` statements* expression `}`

unwrap_assign:
    \ namespace `(` ~ name_multi `)` `=` expression
    \ namespace `{` ~ name_multi `}` `=` expression

name_multi:
    \ name (`,` name)*
```

## Assignment

```
assignment:
    \ field_multi (`+=` \ `-=` \ `*=` \ `/=` \ `=`) expression
    \ field_multi `=` expression

field_multi:
    \ field (`,` field)*

field:
    \ field `.` ~ name
    \ name
```

## Expression

```
expression:
    \ expression (`and` \ `or` \ `xor`) expr_comp
    \ expr_comp

expr_comp:
    \ expr_comp (`>=` \ `<=` \ `==` \ `!=` \ `>` \ `<`) expr_add
    \ expr_add

expr_add:
    \ expr_add (`+` \ `-`) expr_mul
    \ expr_mul

expr_mul:
    \ expr_mul (`*` \ `/` \ `%`) expr_unary
    \ expr_unary

expr_unary:
    \ (`+` \ `-` \ `not`) ~ expr_unary
    \ expr_subscript

expr_subscript:
    \ expr_primary `[` ~ expression `]`
    \ expr_primary

expr_primary:
    \ `(` ~ expression `)`
    \ expr_member
    \ string
    \ integer
    \ decimal
    \ boolean

expr_func_call:
    \ expr_func_call `(` ~ args? `)` `?`?
    \ expr_get_member

expr_get_member:
    \ expr_get_member `.` ~ name
    \ expr_func_call
    \ expr_namespace

args:
    \ expression (`,` expression)*

expr_namespace:
    \ expr_namespace `::` name
    \ name

name:
    \ `a` \ ... \ `z`
    \ `A` \ ... \ `Z`
    \ `_`
```

## Boolean

```
boolean:
    \ `true`
    \ `false`
```

## Number

```
decimal:
    \ !`0` digit+ `.` digit+

integer:
    \ `0` ~ `x` hex+
    \ digit+

hex:
    \ digit
    \ `a` \ ... \ `f`
    \ `A` \ ... \ `F`

digit:
    \ `0` \ ... \ `9`
```

## String

```
string:
    \ `f` ~ `"` char_format* `"`
    \ `r` ~ `"` char* `"`
    \ `"` char_escape* `"`

char_format:
    \ `{{`
    \ `}}`
    \ `{` ~ expression? `}`
    \ char_escape

char_escape:
    \ `\` ~ (`r` \ `n` \ `t` \ `v` \ `f` \ `\`)
    \ char

char:
    \`U+000000` \ ... \ `U+10FFFF`
```
