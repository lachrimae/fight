.PHONY: default
default:
	RUSTFLAGS="-L /usr/local/lib/libfmod.so -L /usr/local/lib/libfmodstudio.so" cargo build --package fight-client

.PHONY: play
play:
	RUSTFLAGS="-L /usr/local/lib/libfmod.so -L /usr/local/lib/libfmodstudio.so" cargo run --package fight-client

.PHONY: play
play-release:
	RUSTFLAGS="-L /usr/local/lib/libfmod.so -L /usr/local/lib/libfmodstudio.so" cargo run --package fight-client --release
