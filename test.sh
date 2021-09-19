#!/bin/bash

test() {
    input=$1
    expect=$2

    ./target/debug/rcc $input
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
test 1 1
test 11 11
test 45 45
# test 4+5 9
