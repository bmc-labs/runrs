name: Checks

on:
  pull_request:
    branches: [ "trunk" ]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check --all
      - run: cargo clippy --all-targets -- -D warnings -D clippy::all

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@nextest
      - uses: Swatinem/rust-cache@v2
      - run: cargo nextest run --cargo-profile release
        env:
          RUST_BACKTRACE: 1

  typos:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: crate-ci/typos@v1.18.0
