# Felys Grammar Future Design

Here are some immature design of Felys. PEG parser does not support left recursion, a modified packrat parser (PEG with a caching model) does. This is exactly what CPython did make the parser express programmers' intention better.

## Assignment

```
assignment:
    \ field (`+=` \ `-=` \ `*=` \ `/=` \ `=`) expression

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
    \ expr_func_call `(` ~ args? `)`
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
