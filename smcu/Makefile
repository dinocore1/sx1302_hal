.PHONY: all clean

all: target/debug/libsmcu.a
	

clean:
	cargo clean
	rm -f unit_test

target/debug/libsmcu.a:
	cargo build
	cbindgen --config cbindgen.toml --crate smcu --output inc/smcu.h

unit_test: test/test.cc target/debug/libsmcu.a
	$(CXX) -o $@ -Iinc -L target/debug -g -O0 $< -lsmcu -lgtest_main -lgtest -lpthread -ldl