
RUSTC_REL=/opt/rust-0.2
RUSTC_HEAD=/opt/rust-HEAD
RUSTC_ROOT=$(RUSTC_HEAD)
RUSTC=$(RUSTC_ROOT)/bin/rustc

.PHONY: all gtk clean

all: gtk

gtk:
	$(RUSTC) gtk.rc

clean:
	-rm *.o gtk

