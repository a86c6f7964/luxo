.PHONY: info devdeps fmt

info:
	@echo "Use fmt target to format code"

devdeps:
	rustup component add rustfmt-preview --toolchain=nightly
	
fmt:
	cargo +nightly fmt
