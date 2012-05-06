
RUSTC=rustc
RUSTC_OPTS=-L ../template

.PHONY: all clean

all: main

main: main.rs
	$(RUSTC) $(RUSTC_OPTS) main.rs

clean:
	-rm *.o main
	-rm -rf *.dSYM

