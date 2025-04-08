# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

# just list all targets
default:
    @just --list

alias c  := check
alias ut := unit-test
alias it := integration-test

alias qa := release-qa

alias db := debug-build
alias rb := release-build

# Install development version
install:
    cargo install --locked --path tackler-cli

# clean the workspace
clean:
    cargo clean

# run code style and linter checks
check: clippy
    cargo fmt --all --check -- --style-edition 2024

# run clippy the linter
clippy:
    cargo clippy --workspace --all-targets --no-deps -- -D warnings

# format code
fmt:
    cargo fmt --all -- --style-edition 2024

# run all tests
test: (_test "debug")

# run ci tests
release-qa: clean check audit (_test "release")

_test target: unit-test (_build target) (_examples-test target) (_integration-test target)

# run unit tests
unit-test:
    cargo test

# run integration tests (the shell runner)
integration-test: (_build "debug") (_integration-test "debug")

# run examples
examples: (_build "debug") (_examples-test "debug")

_integration-test target:
    bash tests/sh/test-runner-ng.sh --{{target}}

_examples-test target: (_build target)
    target/{{ target }}/tackler --config "{{justfile_directory()}}/examples/simple.toml"
    target/{{ target }}/tackler --config "{{justfile_directory()}}/examples/audit.toml"
    target/{{ target }}/tackler --config examples/maple.toml
    target/{{ target }}/tackler --config examples/solar.toml
    target/{{ target }}/tackler --config examples/solar.toml --pricedb examples/solar/txns/se-sold.db

_build target:
    @if [ "debug" = "{{ target }}" ]; then cargo build --bin tackler; else cargo build --release --bin tackler; fi

# build the debug target
debug-build: (_build "debug")

# build the release target
release-build: (_build "release")

# run integration level benchmarks
bench: _git_bench
    cargo bench parser

_git_bench:
    cargo run --release -p tackler-core

# run audit checks (advisories, bans, licenses, sources)
audit:
    cargo deny check advisories bans licenses sources

