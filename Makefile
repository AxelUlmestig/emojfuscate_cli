.PHONY: install-executable
install-executable:
	./scripts/install.sh

.PHONY: test
test:
	cargo build
	bats test.bats
