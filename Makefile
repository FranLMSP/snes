test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features

run:
	cargo run --bin snes-frontend

run-release:
	cargo run --bin snes-frontend --release

