all: deps
	./build-styles.sh
	cargo web deploy

deps:
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	cargo --version
	cargo install cargo-web --force
	npm install -g dart-sass
