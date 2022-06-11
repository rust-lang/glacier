# Glacier

A big ‘ol pile of ICE.

[![Build Status](https://github.com/rust-lang/glacier/workflows/Continuous%20Integration/badge.svg)](https://github.com/rust-lang/glacier/actions)

This repository is used to test internal compiler errors (also known as ICEs)
in [Rust]. An ICE means that something went wrong, something unexpected. As
such, sometimes, ICEs get randomly fixed. A compiler refactoring will remove
the odd code path, even if the point of the change wasn’t to fix the bug.

[Rust]: https://github.com/rust-lang/rust

As such, this repository is a collection of these bugs, and it runs on Rust
nightly, once a day, through GitHub Actions. If any of the ICEs stop happening, the build
will fail, and we can close the associated bug.

## Helping out

Contributing to Glacier is fairly easy:

1. Check out [this list][ices] of bugs on the Rust issue tracker.
2. Pick one.
3. Create a file in `ices/` with the same digit as the bug.
4. Copy the code that causes the ICE into your new file.
5. (optional) Verify it works by running `rustup update nightly`, then `cargo run` to run the tests.
6. Send a pull request!

Note: Running this on Windows may give false positives and report some ICEs as fixed,
use either WSL or Linux for better accuracy.

[ices]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+label%3AI-ICE+-label%3AE-needs-mcve+-label%3Aglacier+-label%3Arequires-debug-assertions+

## License

MIT/Apache2, just like Rust.
