#!/bin/bash

# Run formatting check
cargo fmt -- --check

# Run typo check
typos

# Run clippy
cargo clippy -- -D warnings