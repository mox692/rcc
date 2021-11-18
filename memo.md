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


10/28
* local valをどの様にcountするか.
  * やり方としては2つ考えていて、
    * parse()にglobalな変数を渡して、local_valが見つけられるたびにincre.
    * debug_nodesみたいに、作成されたnodesを辿っていく処理を追加
  * 個人的には2つめのやり方の方が汚くならない気がするし、後に最適化pathをいれる場合にも、そこの処理が使えそう.
  * debug_nodesに追加する形でやってもいいかも

11/2
* function内のlocalvalをcountする機構を作成した
* 次に、変数宣言をサポートしていく予定(ステップ17)

11/6
* block_str変数を出せるようにした.
* ただ、やはり表にしないと意味がない気がするので、次回は表を作る部分の実装
  * codegenはblock nodeを直接見るわけではなく、あくまで識別子(ident)しか見ていない
  * そのため、識別子のnodeに直接block_strを付与するように
  * あと、同じblock内で変数がhitしなかった際の、1つ上のnodeを探す際とかも、tableになっていないと意味がない気がする.
    * これはcodegen側でやるtaskかも.

11/11
* read_nodeをリファクタして、引数等をまとめたstructを作る.
* ↑のstruct にcur_blockという、現在読んでるblock strをいれるfieldを作る
* ident_nodeがきた時に、block_strを各処理を追加
  * block node -> block node strを更新
  * idnet_node -> nodeにstrを書き込み、関数内でglobalなidnet tableへの書き込み.
ここまでやると、

11/13
* block内で同じsymbol名を使えないようにした.
  * nextとしては
    * int で、変数宣言の概念を導入
    * (最上位のblockじゃない時に、)同階層にsymbolがなくても、上のblockを順にsearchしていく機能を付け足す.
      * FunctionLocalVariableのmethodに実装する感じかな.

11/15
* declとassignを明確に区別する
### TODO
* 変数scopeが何やらおかしい.
  * block
    * 下記はとりあえず落ちた
    * scope関連は、さしあたりの目標はfunction scopeにしたい.
    * function作る時にまた考える.
* 変数宣言と、代入を分ける
  * a = 3; とした時に、今は宣言か代入かわからない
  * 宣言の場合は、その変数のscopeも記録するようにする.
    * https://godbolt.org/ で試して見た結果、
      * local変数をカウントするのは、function毎(blockごとにはしてないみたい)
      * block内で同じ変数名で再び宣言があったら、別の変数として扱う
        * その前にblockというnodeを追加する
          * blockにはindexを表現したstringを保存するようにする
  * `int a = 44;`みたいに、はじめは全てint型にする.
* 複数のfunctionをparseできるように
  * `int hoge() {}`という構文を新しく追加する必要がある.
  * `int main() {}`がないとerror.
* function callをできるように.
  * `hoge()`でhogeの定義にjmpするように
  * local変数の時と同様に、fucntion table的なものを作成した方がいいかも
      
```
a = 4;
b=3;
c=1;
if(a<2){
    b=3;
    c = 2;
    if(a > 2){
        3;
    }
} else if (a == 3) {
    return 32;
} else {
    c = 2;
    if(c > a) {
        return 22;
    } else {
        a = c+b;
        return a;
    }
}
```
* Error処理
  * parser, codegen, tokenizerでそれぞれ違ってくるかも.
* for文の最後の構文が、(再代入できない故)exprになっている
  * i++みたいなことは現状できなくなっている
  * supportした後に、ここの文法は改める.
* for文とかif文のstmtsは、program(stmts*)では現状ダメということになっている.
  * これも明らかにおかしい.
* for文の3つの要素は、普通のC言語では任意(あってもなくても良い、for(;;)みたいな書き方ができる)だが、今は3つがないとできない.

### 変数scope
下記のelse blockにおいて、
```
int a = 2;
if (a < 1) {
  ...
} else {
  int a = 5;
  a = 0;
  return a;
}
```
`return a;` の`a`が、きちんとblock内で宣言された`a`を評価するようにしたい.
そのために、下記の仕組みを設けることを考えている.

* parser
  * blockというnodeを作成し、所属しているfunctionからのindex,深さを保存する
  * IDENT_NODEを作成時に、そのIDENT_NODEが生成されたblockのindex,深さをメモしたparameterを埋める.
* intermediate
  * 変数宣言が同じblock内でされていたら、errorにする
* codegen
  * IDENT_NODEをcodegen時、parameterをkeyに
  * 変数を評価するとき(変数名で検索をかける)、現在のblockから順番に、上の深さに向かって検索していく

### Current Syntax
```
source = program
program = function
function = int ident "(" ")" block
stmts = ( stmts2 | ifstmt | forstmt)
stmts2 = block | stmt
block = "{" stmts* "}"
forstmt = "for" "(" declare ";" equality ";" expr ")" stmts2
ifstmt = "if" if_node ( elsif_node )? ( else_node )?
if_node = "(" if_cond ")" stmts2
elsif_node = "else if" "(" if_cond ")" stmts2
else_node = "else" stmts2
if_cond = equalit
stmt = ( declare | assign | return | equality ) ";"
declare = &type &ident "=" equality
type = "int"
return = "return" equality
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )?
assign = &ident "=" equality 
expr = add_sub
add_sub = mul_div( "+" mul_div | "-" mul_div )*
mul_div = unary ( "*" unary | "/" unary )*
unary = &num | &ident | &ident "(" ")"
```

* 更新
  * 1105: block node追加
  * 1116: intの変数宣言.
  * 1117: parserの処理対象をfunctionに変更

  