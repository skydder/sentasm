prep:make-target.sh
	./make-target.sh

test-all:test test-5cc


test:tests/test.sh prep
	./tests/test.sh

test-5cc:tests/test2.sh 5cc prep
	./tests/test2.sh

5cc:
	./tests/clone-5cc.sh

fizzbuzz:
	./tests/fizzbuzz/5cc.sh
.PHONY: test test-5cc test-all prep fizzbuzz