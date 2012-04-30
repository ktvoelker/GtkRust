
RUSTC=rustc
RUSTDOC=rustdoc
RUSTDOC_OPTS=--output-dir . --output-format markdown --output-style doc-per-crate

.PHONY: all doc gtk clean

all: gtk doc

gtk:
	$(RUSTC) gtk.rc

doc:
	$(RUSTDOC) $(RUSTDOC_OPTS) gtk.rc

clean:
	-rm *.o gtk gtk.md

