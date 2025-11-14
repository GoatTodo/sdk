# GoatTodo SDK

This repository houses the cross platform GoatTodo SDK, written in Rust.

## Project Goals

🚨 This is not yet ready for public use 🚨

I'm ([coltonhurst](https://github.com/coltonhurst)) building a Todo SDK in [Rust](https://www.rust-lang.org) for educational purposes.

Current project goals:
- Create an ergonomic SDK for todo apps
- Write the entire SDK by hand in pure Rust, using zero dependencies except for crypto related dependencies (currently only: `uuid`)

## SDK Usage

This sdk is currently only published on [crates.io](https://crates.io/crates/goattodo-sdk).

Because this project is not ready for public use, only a `hello_world()` function is public.

## SDK Development

### Build

`cargo build`

### Test

`cargo test`

### Format

`cargo +nightly fmt`

(You may need: `rustup component add rustfmt --toolchain nightly`)

## Versioning

This sdk follows [semver](https://semver.org). Anything before major version `1.0.0` is subject to breaking changes.

## License

This entire repository, all contributions, use, and more are licensed under the [AGPLv3](./LICENSE) license. You are free to use this project or contribute to this project as long as this license is followed; helpful information may be found [here](https://choosealicense.com/licenses/agpl-3.0).
