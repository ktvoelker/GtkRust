
LIBS=gtk2
SAMPLES=demo mem

.PHONY: all doc libs samples $(LIBS) $(SAMPLES) clean

libs: $(LIBS)

all: doc libs samples

doc:
	cd gtk2 && make doc

samples: $(SAMPLES)

demo: gtk2
	cd demo && make

mem:
	cd mem && make

gtk2:
	cd gtk2 && make

clean:
	-cd mem && make clean
	-cd demo && make clean
	-cd gtk2 && make clean

