.PHONY: start debug

start:
	TRACY_NO_INVARIANT_CHECK=1 cargo run

debug:
	cargo run --features "bevy/dynamic debug"

