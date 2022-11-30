Send HTTP requests through Arti client (Tor)

    cargo build
    cargo run -- --host <hostname>

# First go at Arti Client

🦀 Attempts to be a rustacean.

## Notes:

There's a note on the docs for this client that references tor client needing to be wrapped in an async function. So if you do not understand async Rust as much as you like. Go back and go over that first. This is a guide from 2020 but it helped a bunch for me: https://medium.com/@alistairisrael/demystifying-closures-futures-and-async-await-in-rust-part-3-async-await-9ed20eede7a4
- https://tpo.pages.torproject.net/core/doc/rust/arti_client/index.html#connecting-to-tor
- Tokio crate to create a sane pattern by making the main function async.

- I initially used `Result<(), Box<dyn std::error::Error` as the output pattern for this arti_client's implementation of Result and Error. But, as noted in Rust docs and using this [Error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html), "The drawback is that the underlying error type is only known at runtime and not statically determined.". So I created my own Error type with `thiserror` and `anyhow` crates.
