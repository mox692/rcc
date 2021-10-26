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

# exit code test.
test "42;" 42

# add_sub support.
test "4+5;" 9
test "12+34+31;" 77
test "5-2;" 3
test "15-2-7;" 6
test "13+33-12+1;" 35

# mul, div support.
test "4 * 2;" 8
test "3 + 4 * 2;" 11
test "6 / 2;" 3
test "10/ 2/5;" 1
test "10+3*2-     6;" 10
test "15* 2+16/  4;" 34

# multiple expression.
test "4; 5;" 5
test "15* 2+16/  4; 10+3*2-     6;  3 + 4 * 2;" 11

# local val.
test "a = 3; a;" 3
test "abf = 123; abf;" 123
test "A = 2; B = 3; C = 4; D= A+C; E=B+D; F=E/3; F+A*B;" 9 
test "a = 2; b = 3; a*b;" 6
test "a = 2; b = 3; a*b; a+b+b;" 8

# return stmt.
test "return 3;" 3
test "return 3; return 4;" 3
test "aa = 4; return aa + 3;" 7
test "p = 3; q = 5; return p + q; q * p;" 8

# equalities.
test "3 == 3;" 1
test "2 == 0;" 0
test "a = 3; b = 1; a != b;" 1
test "a = 3; b = 1; return a != b; return a;" 1
test "4 < 5;" 1
test "4 < 2;" 0
test "4 > 2;" 1
test "4 > 5;" 0
test "4 >= 3;" 1
test "5 <= 5;" 1
test "a = 5 <= 5; b = 4; a + b;" 5

# if statement.
test "if (2 < 5) 34;" 34
test "if (3 > 2) a = 2; a;" 2
test "a = 3; b = 2; if (a * b > a + b) return 3; c = 32; return c + c;" 3
test "a = 3; b = 2; if (a * b < a + b) return 3; c = 32; if (c < b * a) return c + b; d = 22; d + c;" 54
test "if (2 < 5) 33; else if (3 < 2) 34;" 33
test "if (2 > 5) 33; else 4; " 4
test "if (2 > 5) 33; else a = 4; return a;" 4

# for statement.
test "for (a = 2;  10 < 3; 3+2;) b = 3; 2; return b;" 3
# test "for (a = 2;  10 < 3; 3+2;) for(c = 0; c > 10; 3;) d = 3; 2; return d;" 3

# block
test "{3; 3;3; } return 3;" 3
test "if(3>2){a=3;if(a > 2){3;} }" 3

