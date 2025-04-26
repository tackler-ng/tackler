# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

j := quote(just_executable())

# List all targets
default:
    @{{ j }} --list --unsorted

alias c := check
alias ut := unit-test
alias it := integration-test
alias qa := release-qa
alias db := debug-build
alias rb := release-build
alias help := default

# Clean workspace
clean:
    cargo clean

# Format code
fmt:
    {{ j }} --fmt --unstable
    cargo fmt --all -- --style-edition 2024

# Run audit checks (advisories, bans, licenses, sources)
audit:
    cargo deny check advisories bans licenses sources

# Run code style and linter checks
check: clippy
    cargo fmt --all --check -- --style-edition 2024

# Run clippy the linter
clippy:
    cargo clippy --workspace --all-targets --no-deps -- -D warnings

# Fix with clippy the linter
fix *ARGS:
    cargo clippy --workspace --all-targets --no-deps --fix {{ ARGS }}

# Run unit tests
unit-test *ARGS:
    cargo test {{ ARGS }}

# Run integration tests (the shell runner)
integration-test: (_build "debug") (_integration-test "debug")

# Run examples
examples: (_build "debug") (_examples-test "debug")

# Run all tests
test: (_test "debug")

_test target: unit-test (_build target) (_examples-test target) (_integration-test target)

# Run QA target (for release)
release-qa: clean audit check (_test "release") git-bench

# Build debug target
debug-build: (_build "debug")

# Build release target
release-build: (_build "release")

# Install development version from working copy
install:
    cargo install --locked --path tackler-cli

# Run all benchmarks
bench: parser-bench git-bench

# Run parser benchmarks
parser-bench:
    cargo bench parser

_integration-test target:
    bash tests/sh/test-runner-ng.sh --{{ target }}

_examples-test target: (_build target)
    target/{{ target }}/tackler --config "{{ justfile_directory() }}/examples/simple.toml"
    target/{{ target }}/tackler --config "{{ justfile_directory() }}/examples/audit.toml"
    target/{{ target }}/tackler --config examples/maple.toml
    target/{{ target }}/tackler --config examples/solar.toml
    target/{{ target }}/tackler --config examples/solar.toml --pricedb examples/solar/txns/se-sold.db

_build target:
    @if [ "debug" = "{{ target }}" ]; then cargo build --bin tackler; else cargo build --release --bin tackler; fi

# Run git benchmark and tests
git-bench:
    cargo run --release -p tackler-core
