.PHONY: all clean

all: libsmcu.a

clean:
	cargo clean
	rm -f unit_test

libsmcu.a: src/*.rs
	cargo build
	cbindgen --config cbindgen.toml --crate smcu --output inc/smcu.h
	cp target/debug/libsmcu.a $@

unit_test: test/test.cc libsmcu.a
	$(CXX) -o $@ -Iinc -L. -g -O0 $< -lsmcu -lgtest_main -lgtest -lpthread -ldl