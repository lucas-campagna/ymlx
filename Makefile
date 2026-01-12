# YMX Makefile
# Provides simple build targets for YMX components

.PHONY: help build test clean install cli wasm library docker package all

# Default target
help:
	@echo "YMX Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build    - Build all components"
	@echo "  cli      - Build CLI binary"
	@echo "  wasm     - Build WASM module"
	@echo "  library  - Build static library"
	@echo "  docker   - Build Docker images"
	@echo "  package  - Create distribution package"
	@echo "  test     - Run tests"
	@echo "  clean    - Clean build artifacts"
	@echo "  install  - Install CLI locally"
	@echo "  all      - Build everything"
	@echo ""

# Build all components
all: test cli wasm library package

# Basic build
build: cli

# Build CLI binary
cli:
	@echo "Building CLI..."
	cargo build --release --bin ymx
	@echo "CLI binary: target/release/ymx"

# Build WASM module
wasm:
	@echo "Building WASM module..."
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		echo "Installing wasm-pack..."; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	fi
	wasm-pack build --target web --out-dir pkg --release
	@echo "WASM module: pkg/"

# Build static library
library:
	@echo "Building library..."
	cargo build --release --lib
	@echo "Library: target/release/libymx.a"

# Build Docker images
docker:
	@echo "Building Docker images..."
	docker build -f Dockerfile.cli -t ymx-cli .
	docker build -f Dockerfile.wasm -t ymx-web .
	@echo "Docker images built"

# Create distribution package
package: cli library
	@echo "Creating package..."
	@mkdir -p dist/ymx-latest
	@cp target/release/ymx dist/ymx-latest/
	@cp target/release/libymx.a dist/ymx-latest/
	@cp README.md dist/ymx-latest/
	@if [ -d "pkg" ]; then cp -r pkg dist/ymx-latest/wasm; fi
	@cd dist && tar -czf ymx-latest.tar.gz ymx-latest/
	@echo "Package: dist/ymx-latest.tar.gz"

# Run tests
test:
	@echo "Running tests..."
	cargo test --verbose
	@echo "Tests passed"

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cargo clean
	rm -rf dist/
	rm -rf pkg/
	@echo "Clean complete"

# Install CLI locally
install: cli
	@echo "Installing CLI..."
	cp target/release/ymx /usr/local/bin/ || \
		sudo cp target/release/ymx /usr/local/bin/
	@echo "CLI installed to /usr/local/bin/ymx"

# Development server for WASM
serve-wasm:
	@echo "Starting development server..."
	@if [ -d "pkg" ]; then \
		cd pkg && python3 -m http.server 8080; \
	else \
		echo "WASM module not built. Run 'make wasm' first."; \
	fi

# Check code quality
check:
	@echo "Checking code quality..."
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "Code quality check passed"

# Benchmark
benchmark:
	@echo "Running benchmarks..."
	cargo bench || echo "No benchmarks found"

# Documentation
docs:
	@echo "Building documentation..."
	cargo doc --all-features --no-deps
	@echo "Documentation: target/doc/"

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit || echo "Install cargo-audit for security checks"