.PHONY: start debug

start:
	cargo run --features bevy/dynamic

debug:
	cargo run --features "bevy/dynamic debug"

