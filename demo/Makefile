
RUSTC=rustc
RUSTC_OPTS=-L ../gtk2

.PHONY: all main clean

all: main

main: main.rs
	$(RUSTC) $(RUSTC_OPTS) main.rs

clean:
	-rm *.o main
	-rm -rf *.dSYM

