SHELL := /bin/bash

all: deps
	./build-styles.sh
	# Source cargo file
	source ${HOME}/.cargo/env
	cargo web deploy

deps:
	# Get Rust toolchain via rustup
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	# Activate toolchain
	source ${HOME}/.cargo/env
	# Confirm it works
	cargo --version
	# Get any needed cargo libraries
	cargo install cargo-web --force
	# Get style deps
	npm install -g dart-sass
