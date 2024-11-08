#! /bin/bash

run() {
    cargo run $1
}

test() {
    expected="$2"
    run tests/test1/test$1.asm > tests/target/test1/test$1.S
    nasm -f elf64 -o tests/target/test1/test$1.o tests/target/test1/test$1.S
    ld -o tests/target/test1/test$1 tests/target/test1/test$1.o -m elf_x86_64
    ./tests/target/test1/test$1
    actual="$?"
    if [ "$actual" = "$expected" ]; then
        echo "test$1 => Ok($actual)"
    else
        echo "test$1 => Err($expected expected, but got $actual)"
        exit 1
    fi
}

test 2 42
test 3 55
test 4 7
test 5 0
echo Ok