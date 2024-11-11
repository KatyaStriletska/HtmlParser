format:
	cargo fmt

clippy:
	cargo clippy

run: 
	cargo run -- parse $(FILE)

test:
	cargo test 

build:
	cargo build

help:
	cargo run -- --help

credits:
	cargo run credits

parse: 
	cargo run -- parse $(FILE)