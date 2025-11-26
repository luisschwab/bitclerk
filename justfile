alias b := build
alias c := check
alias f := format

_default:
    @just --list

# Build code
build:
   cargo build

# Check code: formatting, compilation and linting
check:
   cargo +nightly fmt --all -- --check
   cargo check

# Format code
format:
   cargo +nightly fmt
