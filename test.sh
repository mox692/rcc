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