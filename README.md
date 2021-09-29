C compiler written in Rust.

### current syntax
·EBNF like expression
```
source = program
program = stmt*
stmt = ( assign | return | equality | ifstmt ) ";"
return = "return" equality
ifstmt = "if" "(" equality ")" stmt 
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )*
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