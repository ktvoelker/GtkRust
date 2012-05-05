
RUSTC=rustc
RUSTC_OPTS=

RUSTDOC=rustdoc
RUSTDOC_OPTS=--output-format html
RUSTDOC_CRATE=--output-dir . --output-style doc-per-crate
RUSTDOC_MOD=--output-dir doc --output-style doc-per-mod

CRATE=template.rc
SOURCES=$(wildcard *.rs)
INPUTS=$(CRATE) $(SOURCES)
CRATE_DOCS=template.html

.PHONY: all doc crate-doc mod-doc clean

lib: $(INPUTS)
	$(RUSTC) $(RUSTC_OPTS) $(CRATE)

all: lib crate-doc mod-doc

doc: crate-doc mod-doc

crate-doc: $(CRATE_DOCS)

mod-doc: doc/index.html

%.html: %.rc
	$(RUSTDOC) $(RUSTDOC_OPTS) $(RUSTDOC_CRATE) "$*.rc"

doc/index.html: $(INPUTS)
	mkdir -p doc
	$(RUSTDOC) $(RUSTDOC_OPTS) $(RUSTDOC_MOD) $(CRATE)

clean:
	-rm *.o *.dylib *.html doc/*.html
	-rm -rf *.dSYM
	-rmdir doc

