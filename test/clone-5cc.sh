if [ ! -d test/target/5cc/$1 ]; then
    cd test
    git clone https://github.com/skydder/5cc-n.git
fi
