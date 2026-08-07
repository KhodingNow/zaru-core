.PHONY: help test build publish clean

help:
	@echo "Available commands:"
	@echo "  make test      - Run all tests"
	@echo "  make build     - Build the library"
	@echo "  make publish   - Publish to crates.io"
	@echo "  make clean     - Clean build artifacts"

test:
	cargo test --all-features
	cargo fmt -- --check
	cargo clippy -- -D warnings

build:
	cargo build --release
	maturin build

publish:
	# Check version
	@echo "Current version: $$(cargo pkgid | cut -d '#' -f2)"
	@read -p "Continue? (y/n) " -n 1 -r; \
	echo ""; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo publish; \
		maturin publish; \
	fi

clean:
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -rf *.egg-info/
	find . -type d -name __pycache__ -exec rm -rf {} +
