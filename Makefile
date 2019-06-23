##
# like aho corasick but different
#
# @file
# @version 0.1

all: target/release/liblike_aho_corasick_but_different.so

target/release/liblike_aho_corasick_but_different.so: src/lib.rs Cargo.toml
	cargo build --release && strip target/release/liblike_aho_corasick_but_different.so

clean:
	rm -rf target

# end
