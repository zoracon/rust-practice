# First go at Arti Client

## Notes:

There's a note on the docs for this client that references tor client needing to be wrapped in an async function. So if you do not understand async Rust as much as you like. Go back and go over that first.
- https://tpo.pages.torproject.net/core/doc/rust/arti_client/index.html#connecting-to-tor
- I do not understand async Rust as much as I'd like but was able to utilize the Tokio crate to create a sane pattern by making the main function async.
- I used `Result<(), Box<dyn std::error::Error` as the output pattern for this arti_client's implementation of Result and Error. But, as noted in Rust docs and using this [Error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html), "The drawback is that the underlying error type is only known at runtime and not statically determined."
- Successfully calls to icanhazip to return the IP of whatever Tor exit node used
