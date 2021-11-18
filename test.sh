#!/bin/bash

test() {
    input=$1
    expect=$2

    ./target/debug/rcc "$input" "false"
    gcc -o gen gen.s

    ./gen

    result=$?
    if [ $result -eq $expect ]
    then
        echo "$input -> $result ok"
    else
        echo "Fail, expect $expect, got $result"
    fi
}

# # exit code test.
test "int main(){ 42; }" 42

# # add_sub support.
test "int main(){4+5;}" 9
test "int main(){12+34+31;}" 77
test "int main(){5-2;}" 3
test "int main(){15-2-7;}" 6
test "int main(){13+33-12+1;}" 35

# mul, div support.
test "int main(){4 * 2;}" 8
test "int main(){3 + 4 * 2;}" 11
test "int main(){6 / 2;}" 3
test "int main(){10/ 2/5;}" 1
test "int main(){10+3*2-     6;}" 10
test "int main(){15* 2+16/  4;}" 34

# multiple expression.
test "int main(){4; 5;}" 5
test "int main(){15* 2+16/  4; 10+3*2-     6;  3 + 4 * 2;}" 11

# local val.
test "int main(){int a = 3; a;}" 3
test "int main(){int abf = 123; abf;}" 123
test "int main(){int A = 2; int B = 3; int C = 4; int D= A+C; int E=B+D; int F=E/3; F+A*B;}" 9 
test "int main(){int a = 2; int b = 3; a*b;}" 6
test "int main(){int a = 2; int b = 3; a*b; a+b+b;}" 8

# return stmt.
test "int main(){return 3;}" 3
test "int main(){return 3; return 4;}" 3
test "int main(){int aa = 4; return aa + 3;}" 7
test "int main(){int p = 3; int q = 5; return p + q; q * p;}" 8

# equalities.
test "int main(){3 == 3;}" 1
test "int main(){2 == 0;}" 0
test "int main(){int a = 3; int b = 1; a != b;}" 1
test "int main(){int a = 3; int b = 1; return a != b; return a;}" 1
test "int main(){4 < 5;}" 1
test "int main(){4 < 2;}" 0
test "int main(){4 > 2;}" 1
test "int main(){4 > 5;}" 0
test "int main(){4 >= 3;}" 1
test "int main(){5 <= 5;}" 1
test "int main(){int a = 5 <= 5; int b = 4; a + b;}" 5

# if statement.
test "int main(){if (2 < 5) 34;}" 34
# test "int main(){if (3 > 2) {int a = 2;} a;}" 2 -> will panic
test "int main(){int a = 3; int b = 2; if (a * b > a + b) return 3; int c = 32; return c + c;}" 3
test "int main(){int a = 3; int b = 2; if (a * b < a + b) return 3; int c = 32; if (c < b * a) return c + b; int d = 22; d + c;}" 54
test "int main(){if (2 < 5) 33; else if (3 < 2) 34;}" 33
test "int main(){if (2 > 5) 33; else 4; }" 4
test "int main(){if (2 > 5) 33; else int a = 4; return a;}" 4

# for statement.
test "int main(){for (int a = 2;  10 < 3; 3+2;) {int b = 3; 2; return b;}}" 3
# test "int main(){for (int a = 2;  10 < 3; 3+2;){3;} for(int c = 0; c > 10; 3;){int d = 3; 2;} return d;}" 3 -> will panic.

# block
test "int main(){{3; 3;3; } return 3;}" 3
test "int main(){if(3>2){int a=3;if(a > 2){3;} }}" 3

# expected result, 
# a -> _1_1
# b -> _1_1_1
# c -> _1_1_2
# d -> _2
test "
int main(){
    if(3>2)
    { 
        if(3>2)
        { 
            int a = 3;
            3; 
            if(4>3)
            {
                int b = 3;
            }
            if(4>3)
            {
                int c = 3;
            }
        }
    } 
    if(3>2)
    {
        int d = 3;
        3;
        return d;
    }
}
" 3

# val declaration
test "int main(){int a = 3; a = 4; a = 1;return a;}" 1
test "
int main(){
    int a = 3;
    if(a > 2) {
        int b = 3;
        b = 1;
        return b;
    }
}
" 1
# MEMO: block内で宣言されたaがきちんと返されているかどうか。()
test "
int main() {
    int a = 1;
    if (2>4) {
        int a = 3;
        return a;
    } else {
        int a = 5;
        return a;
    }
}" 5

test "
int main() {
    int b = 3;
    int a = 4;

    if(a > b) {
        return 4;
    } else {
        return 3;
    }
}
" 4

test "
int main() {
    int a = 3;
    if(6>3) {
        if(6>3) {
            if(6>3) {
                if(6>3) {
                    return a;
                }
            }
        }
    }
}
" 3

test "
int main() {
    if(6>3) {
        int b = 4;
        if(6>3) {
            if(6>3) {
                if(6>3) {
                    b = 2;
                }
            }
        }
        return b;
    }
}
" 2 