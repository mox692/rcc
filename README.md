C compiler written in Rust.

[minicc](https://github.com/mox692/minicc)の書き直し

### memo
* local変数の戦略.
  * `'a=3; a;'`のcodegen.

```
  .globl main
main:
  push %rbp
  mov %rsp, %rbp
  sub $16, %rsp
  lea -8(%rbp), %rax
  push %rax
  mov $3, %rax
  pop %rdi
  mov %rax, (%rdi)
  lea -8(%rbp), %rax
  mov (%rax), %rax
.L.return:
  mov %rbp, %rsp
  pop %rbp
  ret
```

TODO: 
* 'a=3;'を読めるようにする.
  * local val(a)を読めるように.
  * =をtokenizeできるように
* 'a=3;'をcodegenする
  * local変数の個数をカウントする必要がある.(rspを押し下げる量を決めるため)
    * どっかのnodeに、値として持たせておく.
  * assign式が出てきたらmemに即値を割り当て, stmtで出てきたらmemからreadする.
* 'a=3;'をparseする.
  * codegenを満たすように.