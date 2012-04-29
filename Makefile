
.PHONY: all gtk clean

all: gtk

gtk:
	rustc gtk.rc

clean:
	-rm *.o gtk

