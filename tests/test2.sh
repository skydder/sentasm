prepare_main() {
    cargo run tests/main.asm > tests/target/5cc/main.s
    nasm -f elf64 tests/target/5cc/main.s -o tests/target/5cc/main.o
    gcc -c  -o tests/5cc-n/target/test/common.o -xc tests/5cc-n/test/common
    cd tests/5cc-n/ 
    make test
    cd ../..
}


assemble() {
    # mkdir test/target/5cc/$1
    cargo run tests/5cc-n/target/test/$1.s > tests/target/5cc/$1/nasm
    printf "extern assert_\nextern printf" >> tests/target/5cc/$1/nasm
    nasm -f elf64 tests/target/5cc/$1/nasm  -o tests/target/5cc/$1/elf.o
}

link() {
    ld -m elf_x86_64 -o tests/target/5cc/$1/elf tests/target/5cc/main.o tests/target/5cc/$1/elf.o tests/5cc-n/target/test/common.o -lc -static --dynamic-linker=/lib64/ld-linux-x86-64.so.2
    # ld -m elf_x86_64 -o test/target/5cc/$1/elf test/target/5cc/$1/elf.o test/5cc-n/test/common.o -lc
}

check() {
    objdump -S -s tests/target/5cc/$1/elf > tests/target/5cc/$1/test
    chmod u+x tests/target/5cc/$1/elf
    ./tests/target/5cc/$1/elf
    if [ $? -eq 0 ]; then
        echo "testing $1 ... passed"
    else
        echo "testing $1 ... failed"
        exit 1
    fi
}

test_file() {
    if [ ! -d tests/target/5cc/$1 ]; then
        mkdir tests/target/5cc/$1
    fi
    assemble $1
    link $1
    check $1
}

test() {
    prepare_main
    for file in tests/5cc-n/target/test/*.s; do
        file_name="$(basename $file .s)"
        test_file $file_name
    done
    echo "All tests are completed!!"
}

test