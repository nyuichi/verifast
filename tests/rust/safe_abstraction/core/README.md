This is the Rust standard library's [`core` crate](https://github.com/rust-lang/rust/tree/c871d09d1cc32a649f4c5177bb819646260ed120/library/core/src) (version `nightly-2025-11-25`).

To keep `core`'s `#[path]` includes working, this also vendors `library/stdarch` and `library/portable-simd` from the same Rust commit under `tests/rust/safe_abstraction/`.
