.PHONY: all clean

all:
	cargo build
	cbindgen --config cbindgen.toml --crate smcu --output inc/smcu.h

clean:
	cargo clean