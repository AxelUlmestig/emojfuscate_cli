.PHONY: install-executable
install-executable:
	rm -rf target/release/build/emojfuscate_cli-*
	cargo build --release
	sudo cp target/release/emojfuscate /usr/bin/
	sudo cp target/release/build/emojfuscate_cli-*/out/emojfuscate.bash /etc/bash_completion.d/emojfuscate

.PHONY: test
test:
	cargo build
	bats test.bats
