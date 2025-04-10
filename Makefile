.PHONY: install-executable
install-executable:
	rm -f target/release/build/emojfuscate_cli-*/out/emojfuscate.bash
	cargo build --release
	sudo cp target/release/emojfuscate /usr/bin/
	sudo cp target/release/build/emojfuscate_cli-*/out/emojfuscate.bash /etc/bash_completion.d/emojfuscate

