if [ ! -d test/5cc-n ]; then
    cd test
    git clone https://github.com/skydder/5cc-n.git
    cd 5cc-n
    make 5cc
fi

