# redirect

[![Crates.io](https://img.shields.io/crates/v/redirect.svg)](https://crates.io/crates/redirect)

[Documentation](https://daseinphaos.github.io/doc/redirect/)

`redirect` aims to be a light weight and type safe wrapper around the DirectX12 API for the Rust Programming Language.

**Warning**: This is a Windows 10 only crate. Using under other toolchains would lead to a compilation failure.

## Why Bother

- folks want to work with D3D12
- the Win32 API is ugly to use directly, we want a rusty solution
- wrapping can provide more type safety guarantees

## Current Status

Most of the core API has been ported. Main leftovers include TiledResource, Marker and Event related stuff.
`redirect` can now be used to draw a triangle! Checkout `./examples/triangle.rs`.

## What's Next

- reduce vertex definition boilerplate.
- figure out a way to deal with resources more safely.
- figure out a simple way to do more compile time checking around root signature and pso.
- rework around compute pipeline.
- reconsider safety guarantees.

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
