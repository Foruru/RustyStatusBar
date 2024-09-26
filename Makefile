PREFIX = /usr

all:
	cargo build --release

install:
	mkdir -p ${PREFIX}/bin
	cp ./target/release/rusty_status_bar ${PREFIX}/bin
