C compiler written in Rust.

### Feature.
Still supports basic features only. See [test.sh](https://github.com/mox692/rcc/blob/master/test.sh) to check
currently supported feature.

### Use
Use `echo` command to check exit-code, because this compiler doesn't support print function yet.

```bash
$ cargo build --release

$ ./target/release/rcc ./examples/fib.c     // compile c source.

$ gcc -o gen gen.s                          // assemble & link.

$ ./gen

$ echo $?                                   // check the exit-code
```
