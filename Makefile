SHELL := /bin/bash

all: deps
	./build-styles.sh
	# Source cargo file
	source ${HOME}/.cargo/env
	${HOME}/.cargo/bin/rustup default nightly
	${HOME}/.cargo/bin/cargo web deploy

deps:
	# Get Rust toolchain via rustup
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	# Activate toolchain
	source ${HOME}/.cargo/env
	# Confirm it works
	${HOME}/.cargo/bin/cargo --version
	# Get any needed cargo libraries
	${HOME}/.cargo/bin/cargo install cargo-web --force
	# Get style deps
	npm install -g dart-sass
