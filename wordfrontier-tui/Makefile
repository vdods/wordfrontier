.PHONY: build
build:
	cargo build 2>&1 | sed 's/ *--> wordfrontier-tui\/\(.*\)/\1: error:/g'

.PHONY: run
run:
	cargo run
