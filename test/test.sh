#! /bin/bash

prepare() {
    if [ ! -d "test/target" ]; then
        mkdir test/target
    fi
}

run() {
    cargo run $1
}

test() {
    expected="$2"
    run test/test$1.asm > test/target/test$1.S
    cc -o test/target/test$1 test/target/test$1.S
    ./test/target/test$1
    actual="$?"
    if [ "$actual" = "$expected" ]; then
        echo "test$1 => Ok($actual)"
    else
        echo "test$1 => Err($expected expected, but got $actual)"
    fi
}

prepare
test 2 42
test 3 55
echo Ok