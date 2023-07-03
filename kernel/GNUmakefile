# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

# Default target.
.PHONY: all
all:
	cargo build --target x86_64-unknown-none --profile $(RUST_PROFILE)
	cp target/x86_64-unknown-none/$(RUST_PROFILE_SUBDIR)/limine-rust-barebones kernel.elf

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf kernel.elf

.PHONY: distclean
distclean: clean
