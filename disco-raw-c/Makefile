CC=clang-9
CFLAGS=-I. -std=gnu18

.PHONY: disco build clean-objects post-build-dir clean

disco: main.o
disco: build
disco: clean-objects
disco: post-build-dir

build:
	$(CC) -o disco-raw main.o

clean-objects:
	rm -rf *.o

post-build-dir:
	mkdir -p build
	mv disco-raw build/

clean: clean-objects
	rm -rf build/