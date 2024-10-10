# Makefile

draw:
	cargo run --release

open:
	open image.ppm

test:
	cargo test