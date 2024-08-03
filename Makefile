# Makefile

# Set the name of your binary
BINARY_NAME := secret-manager

# Set the installation directory for the binary
INSTALL_DIR := ${HOME}/.cargo/bin

.PHONY: all build install clean

all: build install clean

build:
	cargo build --release

install:
	@cp ./target/release/$(BINARY_NAME) $(INSTALL_DIR)/sm

clean:
	cargo clean
