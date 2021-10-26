10/3
* codegenする際にif_condのnodeが必要であることが発覚
* 文法を変更.
* cargo run "if (2 < 5) 33; else if (3+3>2) 34;" true でERR
* else ifがtokenizeできていない
* else if tokenを新しく作成したい
* charを塊で読む部分if char.is_ascii_alphabetic() { .... }
  を、関数に切り出したい.

10/5
* やっとif文っぽいものができた
* とはいえサポートしてないパターンもいくつかあり、
  * else ifとelseの併用.(現状併用して書くと、どちらのパスも通ってしまう)
  * else ifの連続使用.
  * if blockなし
  * ifのstmtを1stmtのみに限定している.
* ↑のあたりは今後サポートしていきたい
* とはいえ、ちょっとコードがかなり汚くなってきたので、一旦ここでリファくたしたい.


### TODO
* local valの再代入処理.
  * 同じシンボルに対する再代入が今はできなくてる.
* Error処理
  * parser, codegen, tokenizerでそれぞれ違ってくるかも.
* for文の最後の構文が、(再代入できない故)exprになっている
  * i++みたいなことは現状できなくなっている
  * supportした後に、ここの文法は改める.
* for文とかif文のstmtsは、program(stmts*)では現状ダメということになっている.
  * これも明らかにおかしい.
* for文の3つの要素は、普通のC言語では任意(あってもなくても良い、for(;;)みたいな書き方ができる)だが、今は3つがないとできない.

### Current Syntax
```
source = program
program = stmts*
stmts = ( stmts2 | ifstmt | forstmt)
stmts2 = "{" parse_stmts* "}" | stmt
forstmt = "for" "(" assign ";" equality ";" expr ")" stmts2
ifstmt = "if" if_node ( elsif_node )? ( else_node )?
if_node = "(" if_cond ")" stmts2
elsif_node = "else if" "(" if_cond ")" stmts2
else_node = "else" stmts2
if_cond = equalit
stmt = ( assign | return | equality ) ";"
return = "return" equality
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )?
assign = &ident ( "=" equality )*
expr = add_sub
add_sub = mul_div( "+" mul_div | "-" mul_div )*
mul_div = unary ( "*" unary | "/" unary )*
unary = &num | &ident
```