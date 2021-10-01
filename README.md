C compiler written in Rust.

### current syntax
·EBNF like expression
```
source = program
program = stmts*
stmts = ( stmt | ifstmt )
ifstmt = "if" "(" equality ")" stmts ( "else if" "(" equality ")" stmts ) ( "else" stmts )?
stmt = ( assign | return | equality ) ";"
return = "return" equality
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )?
assign = &ident ( "=" equality )*
expr = ( add_sub | &ident )
add_sub = mul_div( "+" mul_div | "-" mul_div )*
mul_div = unary ( "*" unary | "/" unary )*
unary = &num | &ident
```

### feature.
* Basic calculation(+,-,*,/)
* equalities( ==, !=, <, <=, >, >= )
* local val.
* return stmt.
* if statement.