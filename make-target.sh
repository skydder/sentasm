if [ ! -d target ]; then
    mkdir target
fi

if [ ! -d test/target ]; then
    mkdir test/target
    mkdir test/target/5cc
    mkdir test/target/test1
fi

python src/asmblr/gen_gen.py > src/asmblr/gen_gen.rs