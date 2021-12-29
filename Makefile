MAKEOPTIONS=--no-print-directory

all:
	@make $(MAKEOPTIONS) test

test:
	@cargo test -- --nocapture

doc:
	cargo doc --no-deps --open