#!/bin/bash

# YMX Deployment Script
# This script helps build and deploy different YMX components

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
VERSION=${1:-"latest"}
REGISTRY=${REGISTRY:-"ghcr.io/lucas-campagna/ymx"}

echo -e "${GREEN}YMX Deployment Script${NC}"
echo "Version: $VERSION"
echo "Registry: $REGISTRY"
echo

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to build CLI
build_cli() {
    print_status "Building CLI binary..."
    
    # Build for current platform
    cargo build --release --bin ymx
    
    if [ $? -eq 0 ]; then
        print_status "CLI build successful"
        print_warning "Binary location: target/release/ymx"
    else
        print_error "CLI build failed"
        exit 1
    fi
}

# Function to build cross-platform binaries
build_cross_platform() {
    print_status "Building cross-platform binaries..."
    
    platforms=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin")
    
    for platform in "${platforms[@]}"; do
        print_status "Building for $platform..."
        
        # Install target if not present
        if ! rustup target list --installed | grep -q "$platform"; then
            rustup target add "$platform"
        fi
        
        # Build
        cargo build --release --bin ymx --target "$platform"
        
        if [ $? -ne 0 ]; then
            print_error "Build failed for $platform"
            exit 1
        fi
    done
    
    print_status "Cross-platform build completed"
}

# Function to build WASM
build_wasm() {
    print_status "Building WASM module..."
    
    # Check if wasm-pack is installed
    if ! command -v wasm-pack &> /dev/null; then
        print_warning "wasm-pack not found, installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
    
    # Build WASM package
    wasm-pack build --target web --out-dir pkg --release
    
    if [ $? -eq 0 ]; then
        print_status "WASM build successful"
        print_warning "WASM files location: pkg/"
    else
        print_error "WASM build failed"
        exit 1
    fi
}

# Function to build library
build_library() {
    print_status "Building library..."
    
    cargo build --release --lib
    
    if [ $? -eq 0 ]; then
        print_status "Library build successful"
        print_warning "Library location: target/release/libymx.a"
    else
        print_error "Library build failed"
        exit 1
    fi
}

# Function to generate C headers
generate_headers() {
    print_status "Generating C headers..."
    
    if ! command -v cbindgen &> /dev/null; then
        print_warning "cbindgen not found, installing..."
        cargo install cbindgen
    fi
    
    cbindgen --config cbindgen.toml --crate ymx --output ymx.h
    
    if [ $? -eq 0 ]; then
        print_status "Headers generated successfully"
        print_warning "Header file: ymx.h"
    else
        print_error "Header generation failed"
        exit 1
    fi
}

# Function to build Docker images
build_docker() {
    print_status "Building Docker images..."
    
    # Build CLI image
    docker build -f Dockerfile.cli -t "${REGISTRY}-cli:${VERSION}" .
    
    # Build WASM server image
    docker build -f Dockerfile.wasm -t "${REGISTRY}-wasm-server:${VERSION}" .
    
    if [ $? -eq 0 ]; then
        print_status "Docker images built successfully"
        print_warning "Images: ${REGISTRY}-cli:${VERSION}, ${REGISTRY}-wasm-server:${VERSION}"
    else
        print_error "Docker build failed"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    cargo test --verbose
    
    if [ $? -eq 0 ]; then
        print_status "All tests passed"
    else
        print_error "Tests failed"
        exit 1
    fi
}

# Function to create distribution package
create_package() {
    print_status "Creating distribution package..."
    
    # Create temporary directory
    DIST_DIR="dist/ymx-${VERSION}"
    mkdir -p "$DIST_DIR"
    
    # Copy CLI binary
    cp target/release/ymx "$DIST_DIR/"
    
    # Copy library files
    cp target/release/libymx.a "$DIST_DIR/"
    cp target/release/deps/libymx*.rlib "$DIST_DIR/" 2>/dev/null || true
    
    # Copy WASM files if they exist
    if [ -d "pkg" ]; then
        cp -r pkg "$DIST_DIR/wasm"
    fi
    
    # Copy documentation and examples
    cp README.md "$DIST_DIR/"
    cp -r examples "$DIST_DIR/"
    
    # Copy header if it exists
    if [ -f "ymx.h" ]; then
        cp ymx.h "$DIST_DIR/"
    fi
    
    # Create archive
    cd dist
    tar -czf "ymx-${VERSION}.tar.gz" "ymx-${VERSION}"
    cd ..
    
    print_status "Package created: dist/ymx-${VERSION}.tar.gz"
}

# Function to deploy to staging
deploy_staging() {
    print_status "Deploying to staging..."
    
    # This would typically involve:
    # - Pushing Docker images to registry
    # - Updating staging environment
    # - Running smoke tests
    
    print_warning "Staging deployment not implemented yet"
}

# Function to display usage
usage() {
    echo "Usage: $0 [VERSION] [COMMAND]"
    echo
    echo "Commands:"
    echo "  cli          Build CLI binary"
    echo "  cross        Build cross-platform binaries"
    echo "  wasm         Build WASM module"
    echo "  library      Build library"
    echo "  headers      Generate C headers"
    echo "  docker       Build Docker images"
    echo "  test         Run tests"
    echo "  package      Create distribution package"
    echo "  all          Build everything"
    echo "  staging      Deploy to staging"
    echo
    echo "If no command is specified, 'all' is assumed"
}

# Main logic
if [ $# -eq 0 ]; then
    COMMAND="all"
elif [ $# -eq 1 ]; then
    COMMAND="all"
else
    COMMAND="$2"
fi

case $COMMAND in
    "cli")
        build_cli
        ;;
    "cross")
        build_cross_platform
        ;;
    "wasm")
        build_wasm
        ;;
    "library")
        build_library
        ;;
    "headers")
        generate_headers
        ;;
    "docker")
        build_docker
        ;;
    "test")
        run_tests
        ;;
    "package")
        create_package
        ;;
    "staging")
        deploy_staging
        ;;
    "all")
        print_status "Building all components..."
        run_tests
        build_cli
        build_library
        build_wasm
        generate_headers
        build_docker
        create_package
        print_status "All components built successfully!"
        ;;
    "help"|"-h"|"--help")
        usage
        ;;
    *)
        print_error "Unknown command: $COMMAND"
        usage
        exit 1
        ;;
esac

echo -e "${GREEN}Deployment script completed successfully!${NC}"