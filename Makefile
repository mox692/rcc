TARG=rcc
SRC=./src/main.rs ./src/codegen.rs
test: rcc
	./test.sh

rcc: $(SRC)
	cargo build
