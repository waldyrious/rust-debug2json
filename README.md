# rust-debug2json

A small browser-based utility
that converts Rust [`Debug`][debug-trait] output
to pretty-printed JSON.

It relies on [`serde_dbgfmt`][serde-dbgfmt] for the parsing,
and compiles to a [WebAssembly](https://webassembly.org) module
that is imported via JavaScript into a plain HTML file, for browser usage.

[debug-trait]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[serde-dbgfmt]: https://docs.rs/serde_dbgfmt

## Usage

Visit <https://waldyrious.github.io/rust-debug2json>,
paste your Rust debug output into the left pane,
and click the **Convert to JSON** button.

For local usage, clone the repository
and [build the project](#building-from-source),
then open `index.xhtml` via a local server
(required for the WebAssembly module to load).

## Caveats

This tool works best with standard Rust `Debug` output
— i.e. the format emitted by the [`#[derive(Debug)]`][debug-trait] trait.
Custom `Debug` implementations may produce output that doesn't parse correctly.

A few things to be aware of about how the output maps to JSON:

- Wrapper types like `String("...")` and `Some(...)`
  become single-element arrays in the output, e.g. `["value"]`.
  This is due to how `serde_dbgfmt` deserializes them
  into a generic [`serde_json::Value`][serde-json-value].
- `Object { "key": ... }` wrappers
  (which appear in debug output from `serde_json::Value` itself)
  are automatically stripped before parsing,
  so their contents appear as plain JSON objects.

[serde-json-value]: https://docs.rs/serde_json/latest/serde_json/enum.Value.html

## Building from source

To build this locally, ensure you have [Rust](https://rust-lang.org)
and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) installed.
Note that Rust must be installed via [rustup](https://rustup.rs),
not via Homebrew or a system package manager,
since `wasm-pack` relies on it to manage the WebAssembly compile target.

1. Set up a Rust toolchain via rustup:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install `wasm-pack` to allow compiling Rust to WebAssembly:

   ```sh
   cargo install wasm-pack
   ```

3. Compile the library source code at `src/lib.rs` into WebAssembly module:

   ```sh
   wasm-pack build --target web
   ```

   This generates the `pkg/` directory, which is loaded by `index.xhtml` at runtime.

4. Serve the repository root with any static HTTP server
   (browsers require HTTP, not a `file://` URL, for WebAssembly modules to load).
   For example, using Python's HTTP server module:

   ```sh
   python3 -m http.server
   ```

   Then open http://localhost:8000 in your browser.

## Developing

Contributions are welcome! This project is [licensed](./LICENSE.md) as free software
under the Simple Public License 2.0 (SPDX: `SimPL-2.0`).

If you make changes to the Rust code, make sure to adapt the tests, and run them with:

```sh
cargo test
```

If the new code compiles and the tests pass, rebuild the WebAssebly as per step 3 above.
