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


### refactorポイント
* tokenizer
  * tokenizer専用のstructを作る
    * string, index, curなどをまとめて扱いたい
* parser
  * parse unary, parse binaryなどを止める.
  * 順番を揃える.
  * 
* codegen
  * lv, clをこれまた1つにまとめたい.
* その他
  * readmeの文法とparserの文法があっているか.