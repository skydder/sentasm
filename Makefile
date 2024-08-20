prep:make-target.sh
	./make-target.sh

test:test/test.sh
	./test/test.sh

test-5cc:test/test2.sh 5cc prep
	./test/test2.sh

5cc:
	./test/clone-5cc.sh
