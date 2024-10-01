# Makefile

draw:
	cargo run --release > image.ppm

open:
	open image.ppm

test:
	cargo test