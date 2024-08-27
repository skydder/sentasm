prep:make-target.sh
	./make-target.sh

test-all:test test-5cc


test:test/test.sh prep
	./test/test.sh

test-5cc:test/test2.sh 5cc prep
	./test/test2.sh

5cc:
	./test/clone-5cc.sh

.PHONY: test test-5cc test-all prep