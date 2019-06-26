##
# like aho corasick but different
#
# @file
# @version 0.1

SOEXT := ".so"
ifeq ($(OS),Windows_NT)
	SOEXT = ".dll"
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Darwin)
		SOEXT = ".dylib"
	endif
endif

all: target/release/liblike_aho_corasick_but_different$(SOEXT)

target/release/liblike_aho_corasick_but_different.so: src/lib.rs Cargo.toml
	cargo build --release && strip target/release/liblike_aho_corasick_but_different$(SOEXT)

clean:
	rm -rf target

# end
