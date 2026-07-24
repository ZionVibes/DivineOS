#!/bin/bash

# DivineOS Build Script
# This script automates the build process for DivineOS

set -e  # Exit on any error

echo "=== DivineOS Build Script ==="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust first."
    exit 1
fi

# Check if protoc is available
if ! command -v protoc &> /dev/null; then
    echo "Warning: protoc is not installed. Protocol buffer compilation may fail."
fi

# Build all crates in the workspace
echo "Building all crates..."
cargo build --workspace

# Check for build success
if [ $? -eq 0 ]; then
    echo "✓ All crates built successfully"
else
    echo "✗ Build failed"
    exit 1
fi

# Build in release mode
echo "Building release version..."
cargo build --workspace --release

# Check for release build success
if [ $? -eq 0 ]; then
    echo "✓ Release build completed successfully"
else
    echo "✗ Release build failed"
    exit 1
fi

# Run tests
echo "Running tests..."
cargo test --workspace

# Check for test success
if [ $? -eq 0 ]; then
    echo "✓ All tests passed"
else
    echo "✗ Some tests failed"
    exit 1
fi

# Generate documentation
echo "Generating documentation..."
cargo doc --workspace --no-deps

# Check for documentation success
if [ $? -eq 0 ]; then
    echo "✓ Documentation generated successfully"
else
    echo "✗ Documentation generation failed"
fi

echo "=== Build Process Complete ==="
echo "Build artifacts are located in target/ directory"
echo "Documentation is available in target/doc/ directory"