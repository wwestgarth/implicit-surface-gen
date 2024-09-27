# Makefile

draw:
	cargo run > image.ppm

open:
	open image.ppm

test:
	cargo test