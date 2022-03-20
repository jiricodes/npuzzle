MAKEOPTIONS=--no-print-directory

all:
	@make $(MAKEOPTIONS) test

test:
	@cargo test -- --nocapture

vis:
	cargo run -p npuzzle_vis

doc:
	cargo doc --no-deps --open