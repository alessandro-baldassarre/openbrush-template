# OpenBrush Template

Template for multi-contract [ink-openbrush](https://github.com/727-Ventures/openbrush-contracts) projects

## How to Use

Install [cargo-make](https://sagiegurari.github.io/cargo-make/):

```sh
cargo install --force cargo-make
```

Run formatter:

```sh
cargo make fmt
```

Run tests:

```sh
cargo make test
```

Run linter (clippy):

```sh
cargo make lint
```

Check for unused dependencies:

```sh
cargo make udeps
```

Compile all contracts using [cargo-contract]:

```sh
cargo make build
```

Once optimized, verify the wasm binaries are ready to be uploaded to the blockchain:

```sh
cargo make check
```

Publish contracts and packages to [crates.io](https://crates.io/):

```sh
cargo make publish
```

**NOTE:** For the last task (publish), you need to update the shell script in [`Makefile.toml`](./Makefile.toml) for them to work.

## License

TBD
