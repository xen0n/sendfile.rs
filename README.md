# PROJECT DISCONTINUED

**IMPORTANT** This project is long dead, the `crates.io` package `sendfile` no
longer points at this repo, and is no longer controlled by me.

This library was written during my early Rust days, primarily for learning
purposes, and was never put into production.

Fellow developer [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) has done a
better work of wrapping the nice kernel feature type-safely, at
[`Thomasdezeeuw/sendfile`](https://github.com/Thomasdezeeuw/sendfile).
You can transition to that API for `sendfile` functionality.


# A Platform Sendfile Wrapper for Rust

`sendfile.rs` is a wrapper library aimed to provide zero-copy I/O on platforms supporting it.

This library uses the [`syscall`][syscall-crate] crate to make the system call, which uses inline assembly. Thus, nightly Rust is needed to utilize this library for now.

[syscall-crate]: https://github.com/kmcallister/syscall.rs


## Supported platforms

Currently only Linux is supported; pull requests are welcome, of course.

This library is tested on Linux x86\_64 by the author.


## Usage

Please see the `examples/` directory for usage instructions.


## License

`sendfile.rs` is dual-licensed under either the Apache 2.0 license or the MIT license, which is the same licensing options of Rust.
