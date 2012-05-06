
LIBS=gtk2 parsec template
SAMPLES=demo tdemo mem

.PHONY: all doc libs samples $(LIBS) $(SAMPLES) clean

all: doc libs samples

doc:
	cd parsec && make doc
	cd template && make doc
	cd gtk2 && make doc

clean:
	-cd mem && make clean
	-cd demo && make clean
	-cd gtk2 && make clean
	-cd template && make clean
	-cd parsec && make clean

# Libraries

libs: $(LIBS)

gtk2:
	cd gtk2 && make

template:
	cd template && make

parsec:
	cd parsec && make

# Samples

samples: $(SAMPLES)

tdemo: template
	cd tdemo && make

demo: gtk2
	cd demo && make

mem:
	cd mem && make

