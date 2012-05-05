
LIBS=gtk2
SAMPLES=demo mem

.PHONY: all doc libs samples $(LIBS) $(SAMPLES) clean

demo: gtk2
	cd demo && make

libs: $(LIBS)

all: doc demo libs samples

doc:
	cd gtk2 && make doc

samples: $(SAMPLES)

mem:
	cd mem && make

gtk2:
	cd gtk2 && make

clean:
	-cd mem && make clean
	-cd demo && make clean
	-cd gtk2 && make clean

