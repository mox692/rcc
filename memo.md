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

11/18
* block scopeがやっとできたーーー
* 次はちょっと一旦refactorしたい.

12/16
* func callの実装やる
  * func call nodeの追加 (tokenは新しく追加する必要はなさげ)
    * nodeが持つべきdata
      * lv_size
      * jmp先のlabelかaddr
  * nodeに対しての命令
    * prologue
      * rspとrbpの調整
      * functionのrsp引き下げサイズを把握しておく必要があるな
    * call命令
    * retの埋め込み
    * (関数宣言の時にだけど)local変数みたいに、labelを付与しておく必要がある
  
12/18
* funccallの続き
  * red zoneの存在を一昨日くらいに初めて知って、少し調査してる
  * 参考(red zoneで調べてたらいっぱい出てくる)
    * https://kogara324.hatenablog.com/entry/2019/05/02/045056
    * この挙動はサポートしなくてもいいかもしれない
    * とりあえずlocal変数分stackpointerを下げる挙動にしてみる
* function argsの実装
  * 文法の修正
    * 関数宣言の時に、()内もparse
      * これをlocal変数と同じように扱う
      * (関数呼び出し時に使用する情報として、)
    * caller関数呼び出し時に、引数を特定のレジスタに置くようなcodeを吐く
    * calleeはレジスタに置かれた引数をstackにcopyする処理を追加

* 関数引数のsupport、
  * caller側のcode生成の対応(引数をrdiに入れる)
  * callee側のsupprot(genの前に関数の引数だけregisterからstackに配置しておく)
  * local変数は、intermediate_process.rs内で、関数の引数の後ろに置かれるようになってるはずだから、これでうまく動くはず！....

* 12/20
  * 関数をcodegenする際に引数を表すNodeを含める必要があある気がする.
    * 1. globalなfunction tableみたいなのを作成して、codegenから関数名でaccessできるようにsルウ
    * 2. fn_callnode自体に、引数の情報を持たせるように知る
      * parseの段階は他の関数が見えないから、intermediateで他の関数の引数情報を引っ張ってkルウようにしたい
      * nodeだけparseで作っておいた方がいいかもにした方がいいかも
  * todo
    * fn_call nodeの追加
      * vecでnodeをもつ(a, 34, &bとかが入るよてい)
      * 関数名から他のfnの引数情報を取ってくる
        * fnの引数の情報整理も必要かも
          * args構造体でも作るか
            * type

1/3
* 関数の引数をsupportするようにするぞーーー
* 以下をpassさせる
```
test "
int rec(int a) {
    return a;
}
int main() {
    int b = rec(5);
    return b;
}
" 5
```
* TODO:
  * 宣言とcallにおいて、引数をparseできるように
  * 

1/16
* pointerのtestを通したい！！
```
test "
int foo(int *ptr) {
    int aa = 3 + *ptr;
    return aa;
}
int main() {
    int a = 33;
    int b = foo(&a);
    return b;
}
" 36
```
* よくわからんけど、とりあえずtestが通らなくなってしまっている.
  * ざっくり要約すると、関数の引数に識別子(ident)がある際に、その識別子に対してblock_strを付与してやることができなくなっていた
    * valtableに保存したり取り出したりするのに、block_strが必須.
  * 原因としては関数の引数のnode (fn_arg)が、ident_nodeを取らない構成になっている(ArgsのVecで構成されている)ので、ident_nodeを絡めた構成にする
  * そうすると、intermedeateでblock_strを使用してpointer型を引数に渡すことができるようになるかも.


### TODO
* 複数のfunctionをparseできるように
  * `int hoge() {}`という構文を新しく追加する必要がある.
  * `int main() {}`がないとerror.
* function callをできるように.
  * `hoge()`でhogeの定義にjmpするように
  * local変数の時と同様に、fucntion table的なものを作成した方がいいかも

* print関数を提供する
  * インラインアセンブラをサポートすりゃええやん
* commentのサポート
* CIでformatさせる

* Error msgを豊富にする.
* local valのdebug機構
  * inputをglobalで持ちたい.
    * node探索時のErr(codegenとか)からでも、token列(=入力文字列)でErrを示してあげられるように.
    * lazyStaticとか？


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
    * parse -> tokenReaderにtokのindexをcountさせるようにしといて、そこからErr位置を特定ルウ
    * intermedeate -> 変数重複
      * a
    * codegen -> internalErr?(userErrはintermedeateで全て弾きたい)
    * 全部intermedeateに寄せれない？
      * codegenでやってること
        * label (これのErrって、そもそもcompilerの問題せつだから別で考えてもいいかも)
        * 変数表、offsetの管理
        * シンボル重複、未定義check
### Current Syntax
```
source = program
program = function*
function = int ident "(" ( &type &ident "," )* ")" block
stmts = ( stmts2 | ifstmt | forstmt)
stmts2 = block | stmt
block = "{" stmts* "}"
forstmt = "for" "(" declare ";" equality ";" expr | assign ")" stmts2
ifstmt = "if" if_node ( elsif_node )? ( else_node )?
if_node = "(" if_cond ")" stmts2
elsif_node = "else if" "(" if_cond ")" stmts2
else_node = "else" stmts2
if_cond = equalit
stmt = ( declare | assign | return | equality ) ";"
declare = type ( * )? &ident "=" equality
type = "int"
return = "return" equality
equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )?
assign = &ident "=" equality 
expr = add_sub
add_sub = mul_div( "+" mul_div | "-" mul_div )*
mul_div = unary ( "*" unary | "/" unary )*
unary = &num | &ident | fn_call | ref | deref
ref = "&" &ident
deref = "*" &ident
fn_call = &ident "(" (equality ,)* ")"
```

* 更新
  * 1105: block node追加
  * 1116: intの変数宣言.
  * 1117: parserの処理対象をfunctionに変更
  * 1216: function callに対応の予定
  * 12/18: forのバグ修正による変更(3つめのblockを assign | expr にした)
  * 12/18: 関数引数supportによる変更
  * 1/11: pointer型を作成
  * 1/15: pointer型を作成2

  
### 設計の後悔
* NodeをOption<Box<Node>>にしたのはマジでミスだった.