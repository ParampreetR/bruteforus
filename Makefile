.PHONY: install
install:
	cargo build --release
	strip target/release/bruteforus
	chmod +x ./ci/install.sh
	./ci/install.sh

.PHONY: clean
clean:
	cargo clean
