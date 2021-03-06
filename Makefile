info:
	@echo "Use fmt target to format code"

devdeps:
	rustup component add rustfmt-preview --toolchain=nightly
	command -v hyperfine || cargo install hyperfine

release:
	cargo build --release

bench: release devdeps
	sudo -v
	hyperfine --prepare 'sync && sudo purge' 'target/release/luxo example /tmp/luxo/'
	
fmt:
	cargo +nightly fmt
