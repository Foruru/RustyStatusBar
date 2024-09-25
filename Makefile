PREFIX = /usr/bin

all:
	cargo build --release

install:
	cp ./target/release/rusty_status_bar .
