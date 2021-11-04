.PHONY: build
build:
	cargo test --no-run 2>&1 | sed 's/ *--> wordfrontier\/\(.*\)/\1: error:/g'

.PHONY: test
test:
	RUST_BACKTRACE=1 cargo test

.PHONY: test-nocapture
test-nocapture:
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --nocapture
