.PHONEY: build, run

build:
	cargo build --release

# Cargo will automatically rebuild the project when run with debug profile
debug:
	cargo run $(file)

run:
	cargo run --release $(file)

clean:
	cargo clean