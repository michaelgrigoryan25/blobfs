# This file was generated using the `makegen` cli tool
.PHONY: run release clean

run:
	cargo run

clean:
	python ./scripts/clean.py

release:
	cargo build --release --all-targets
