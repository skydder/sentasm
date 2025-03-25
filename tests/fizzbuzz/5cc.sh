compile() {
    ./tests/5cc-n/5cc tests/fizzbuzz/fizzbuzz.c > tests/fizzbuzz/fizzbuzz.asm
    cargo run tests/fizzbuzz/fizzbuzz.asm > tests/fizzbuzz/fizzbuzz.s
    printf "extern printf" >> tests/fizzbuzz/fizzbuzz.s
    nasm -f elf64 tests/fizzbuzz/fizzbuzz.s -o tests/fizzbuzz/fizzbuzz.o
    ld -m elf_x86_64 -o tests/fizzbuzz/fizzbuzz tests/fizzbuzz/fizzbuzz.o tests/fizzbuzz/t.o tests/target/5cc/main.o -lc -static --dynamic-linker=/lib64/ld-linux-x86-64.so.2
}

compile
./tests/fizzbuzz/fizzbuzz