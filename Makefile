
build:
	cargo build --release

install: build
	cp target/release/kt /usr/local/bin/

