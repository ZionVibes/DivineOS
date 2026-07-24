#!/bin/bash

# DivineOS Test Script
# This script automates testing for DivineOS

set -e  # Exit on any error

echo "=== DivineOS Test Script ==="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust first."
    exit 1
fi

# Run unit tests for all crates
echo "Running unit tests..."
cargo test --workspace --lib

# Check for test success
if [ $? -eq 0 ]; then
    echo "✓ Unit tests passed"
else
    echo "✗ Unit tests failed"
    exit 1
fi

# Run integration tests
echo "Running integration tests..."
cargo test --workspace --features integration

# Check for integration test success
if [ $? -eq 0 ]; then
    echo "✓ Integration tests passed"
else
    echo "✗ Integration tests failed"
    exit 1
fi

# Run documentation tests
echo "Running documentation tests..."
cargo test --workspace --doc

# Check for documentation test success
if [ $? -eq 0 ]; then
    echo "✓ Documentation tests passed"
else
    echo "✗ Documentation tests failed"
fi

# Run specific test suites
echo "Running kernel tests..."
cargo test --package kernel

echo "Running AI runtime tests..."
cargo test --package ai-runtime

echo "Running IPC tests..."
cargo test --package ipc

echo "Running telemetry tests..."
cargo test --package telemetry

echo "Running simulation tests..."
cargo test --package simulation

# Run tests with coverage (if cargo-tarpaulin is available)
if command -v cargo-tarpaulin &> /dev/null; then
    echo "Running code coverage analysis..."
    cargo tarpaulin --workspace --out Xml
    echo "✓ Coverage analysis completed"
else
    echo "Note: cargo-tarpaulin not found, skipping coverage analysis"
fi

echo "=== Test Process Complete ==="