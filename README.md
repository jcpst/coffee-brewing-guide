# coffee-brewing-guide

## Quick Start

Dependencies

* [Rust][] - I recommend [rustup][] to manage the installation
* [Trunk][] - This will build wasm modules and provide a development server

```sh
cd coffee-brewing-guide/coffee-brewing-guide-web-ui
trunk serve
```

Trunk's `serve` subcommand will hot-reload the web page when changes are
made to the source code.

## Project Structure

This repository is setup as a [cargo workspace][] containing two rust projects:
a library, and a WASM binary that brings it in as a dependency. The intention is
to separate the core functionality from any presentation layer early on.

<!-- links -->
[Rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
[Trunk]: https://trunkrs.dev/#install
[cargo workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[jcpst/rust-utility-template]: https://github.com/jcpst/rust-utility-template