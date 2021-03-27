all:
	cargo build --release
	cp ./target/release/iban  /usr/local/bin/