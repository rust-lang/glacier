# Glacier

A big ‘ol pile of ICE.

[![Build Status](https://travis-ci.org/steveklabnik/glacier.svg?branch=master)](https://travis-ci.org/steveklabnik/glacier)

This repository is used to test internal compiler errors (also known as ICEs)
in [Rust]. An ICE means that something went wrong, something unexpected. As
such, sometimes, ICEs get randomly fixed. A compiler refactoring will remove
the odd code path, even if the point of the change wasn’t to fix the bug.

[Rust]: https://github.com/rust-lang/rust

As such, this repository is a collection of these bugs, and it runs on Rust
nightly, once a day, on Travis. If any of the ICEs stop happening, the build
will fail, and I can close the associated bug.

## Helping out

Contributing to Glacier is fairly easy:

1. Check out [this list][ices] of bugs on the Rust issue tracker.
2. Pick one.
3. Create a file in `ices/` with the same digit as the bug.
4. Copy the code that causes the ICE into your new file.
5. (optional) Verify it works by running `cargo run` to run the tests.
6. Send a pull request!

[ices]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AI-ICE+-label%3Aglacier

## License

MIT/Apache2, just like Rust.
