.PHONY: install-executable
install-executable:
	cargo build --release
	sudo cp target/release/emojfuscate /usr/bin/
