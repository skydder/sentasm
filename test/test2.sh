
assemble() {
    mkdir test/target/$1
    cargo run test/5cc-n/target/test/$1.s > test/target/$1/nasm
    nasm -f elf64 test/target/$1/nasm  -o test/target/$1/elf.o 
}

link() {
    cc -o test/target/$1/elf test/target/main test/target/$1/elf.o test/5cc-n/test/common.o
}

check() {
    ./test/target/$1/elf
    if [ $? -eq 0 ]; then
        echo "testing $1 ... passed"
    else
        echo "testing $1 ... failed"
        exit 1
    fi
}

test() {
    assemble arith
    link arith
    check arith
}

test