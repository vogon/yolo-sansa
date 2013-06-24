EXE=./yolo-sansa
SOURCES= \
	crate.rc \
	main.rs \
	lua51.rs
CRATEFILE=crate.rc
RUSTFLAGS=-Z debug-info

.PHONY: all clean

$(EXE): $(SOURCES)
	rustc $(RUSTFLAGS) -o $@ $(CRATEFILE)

all: $(EXE)

clean:
	rm -f $(EXE)