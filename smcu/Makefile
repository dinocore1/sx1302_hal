.PHONY: all clean

all:
	cargo build
	cbindgen --config cbindgen.toml --crate smcu --output inc/smcu.h

clean:
	cargo clean

unit_test: test/test.cc
	$(CXX) -o $@ -Iinc -L target/debug -g -O0 $< -lsmcu -lgtest_main -lgtest -lpthread -ldl