C compiler written in Rust.

### current syntax
·EBNF like expression
```
source = program
program = stmts*
stmts = ( stmt | ifstmt )

ifstmt = "if" if_node ( elsif_node )? ( else_node )?
if_node = "(" if_cond ")" stmts
elsif_node = "else if" "(" if_cond ")" stmts
else_node = "else" stmts
if_cond = equality
stmt = ( assign | return | equality ) ";"
return = "return" equality
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )?
assign = &ident ( "=" equality )*
expr = add_sub
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