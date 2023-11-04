.PHONY: start debug

start:
	cargo run

debug:
	cargo run --features "bevy/dynamic debug"

