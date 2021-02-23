.PHONY: install
install:
	cargo build --release
	strip target/release/bruteforus
	chmod +x install.sh
	./install.sh

.PHONY: clean
clean:
	cargo clean