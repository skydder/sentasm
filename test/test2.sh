prepare_main() {

    cargo run test/main.asm > test/target/5cc/main.s
    nasm -f elf64 test/target/5cc/main.s -o test/target/5cc/main.o 
    cd test/5cc-n/ 
    make test
    cd ../..
}


assemble() {
    # mkdir test/target/5cc/$1
    cargo run test/5cc-n/target/test/$1.s > test/target/5cc/$1/nasm
    printf "extern assert_\nextern printf" >> test/target/5cc/$1/nasm
    nasm -f elf64 test/target/5cc/$1/nasm  -o test/target/5cc/$1/elf.o 
}

link() {
    ld -m elf_x86_64 -o test/target/5cc/$1/elf test/target/5cc/main.o test/target/5cc/$1/elf.o test/5cc-n/test/common.o -lc
    # cc -o test/target/5cc/$1/elf test/target/5cc/$1/elf.o test/5cc-n/test/common.o
}

check() {
    chmod u+x test/target/5cc/$1/elf
    ./test/target/5cc/$1/elf
    if [ $? -eq 0 ]; then
        echo "testing $1 ... passed"
    else
        echo "testing $1 ... failed"
        # exit 1
    fi
}

test_file() {
    if [ ! -f test/target/5cc/$1 ]; then
        mkdir test/target/5cc/$1
    fi
    assemble $1
    link $1
    check $1
}

test() {
    prepare_main
    for file in test/5cc-n/target/test/*.s; do
        file_name="$(basename $file .s)"
        test_file $file_name
    done
}

test