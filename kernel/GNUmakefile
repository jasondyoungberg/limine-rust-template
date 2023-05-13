# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

# Default target.
.PHONY: all
all:
	cargo +nightly --config build.rustflags=\"-C\ no-redzone=y\ -C\ panic=abort\ -C\ relocation-model=pic\ -C\ code-model=kernel\" build -Z build-std=core,compiler_builtins --target x86_64-unknown-none
	cp target/x86_64-unknown-none/debug/limine-rust-barebones kernel.elf

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf kernel.elf

.PHONY: distclean
distclean: clean
