.PHONY: run test
run:
	cargo run -- $(ARGS)
#explain: make run ARGS="$pool_size"
#example: make run ARGS="4"
#example: make run

test:
	cargo test