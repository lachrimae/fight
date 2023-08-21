.PHONY: play
play:
	RUSTFLAGS="-L /usr/local/lib/libfmod.so -L /usr/local/lib/libfmodstudio.so" cargo run --package fight-client
