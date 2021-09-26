C compiler written in Rust.

### current syntax
·EBNF like expression
```
source = program
program = stmt*
stmt = ( assign | expr ) ";"
assign = &ident ( "=" expr )*
expr = ( add_sub | &ident )
add_sub = mul_div( "+" mul_div | "-" mul_div )*
mul_div = unary ( "*" unary | "/" unary )*
unary = &num | &ident
```

### feature.
* Basic calculation(+,-,*,/)