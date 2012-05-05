
LIBS=gtk2 template
SAMPLES=demo mem

.PHONY: all doc libs samples $(LIBS) $(SAMPLES) clean

all: doc libs samples

doc:
	cd gtk2 && make doc
	cd template && make doc

clean:
	-cd mem && make clean
	-cd demo && make clean
	-cd gtk2 && make clean
	-cd template && make clean

# Libraries

libs: $(LIBS)

gtk2:
	cd gtk2 && make

template:
	cd template && make

# Samples

samples: $(SAMPLES)

demo: gtk2
	cd demo && make

mem:
	cd mem && make

