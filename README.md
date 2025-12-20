> [!WARNING]
> This repository is currently in proof of concept status. Intended solely to assess feasibility of moving Kaspa Python SDK into its own repository.
>
> Do NOT use this in production.

# Kaspa Python SDK
The Rusty-Kaspa Python SDK exposes select Rusty-Kaspa source for use in Python applications, allowing Python developers to interact with the Kaspa BlockDAG.

The resulting Python package `kaspa` is a native extensino module, built from Rust wrappers around Rusty-Kaspa's Rust source code using [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs).

> [!WARNING]
> The Kaspa Python SDK is currently in Beta (maybe even Alpha in some regards) status. Please use accordingly.

## Features
One goal for this package is to mirror Kaspa's WASM SDK as closely as possible. From both a feature coverage and usage perspective.

The following main feature categories are currently exposed for use from Python:
- wRPC Client
- Transaction generation
- Key management

This package does not yet fully mirror WASM SDK, gaps mostly exist around wallet functionality. Future work will bring this as close as possible. Potential future features include the ability to read Rusty-Kaspa's RocksDB database, specific exceptions, etc.

## Installing
This package can currently be installed from source or from PyPi. With any crypto project, it's always recommended to install from source when possible.

### Installing from PyPi
`pip install kaspa`

[See the Kaspa package on PyPi here](https://pypi.org/project/kaspa/)

### Installing from Source

1. To build the Python SDK from source, you need to have the Rust environment installed. To do that, follow instructions in the [Installation section of Rusty Kaspa README](https://github.com/kaspanet/rusty-kaspa?tab=readme-ov-file#installation).
2. `cd rusty-kaspa/python` to enter Python SDK crate
3. Run `./build-release` script to build source and built (wheel) dists.
4. The resulting wheel (`.whl`) file location will be printed: `Built wheel for CPython 3.x to <filepath>`. The `.whl` file can be copied to another location or machine and installed there with `pip install <.whl filepath>`

#### `maturin develop` vs. `maturin build`
For full details, please see `build-release` script, `build-dev` script, and [Maturin](https://www.maturin.rs) documentation.

Build & install in current active virtual env: `maturin develop --release --features py-sdk`

Build source and built (wheel) distributions: `maturin build --release --strip --sdist --features py-sdk`.

## Usage from Python

The Python SDK module name is `kaspa`. The following example shows how to connect an RPC client to Kaspa's PNN (Public Node Network).

```python
import asyncio
from kaspa import Resolver, RpcClient

async def main():
    resolver = Resolver()
    client = RpcClient(resolver)
    await client.connect()
    print(await client.get_server_info())

if __name__ == "__main__":
    asyncio.run(main())
```

More detailed examples can be found in `./examples`.

## Bindings Approach/Design

### Overview
This project attempts to leverage native/existing Rusty-Kaspa features as much as possible.

To accomplish that goal, the [newtype pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) is used extensively. Methods of the wrapped RK native struct are then leveraged as much as possible, typically by also wrapping the methods themselves for exposure to Python.

All Python exposed structs and enums defined in this repository are prefixed with `Py` (e.g. `PyRpcClient`). These are then exposed to for use in Python without the prefix.

These `Py` prefixed types exist solely to fulfill Rust/Python interface requirements. When the newtype pattern is used to wrap RK native, these `Py` types are largely wrappers with very little logic, typically just type conversions. When the newtype pattern cannot be used (due to limitations), these `Py` types do more than just type conversions but still attempt to leverage RK native where possible.

This repository relies on various features in RK native that are gated behind WASM feature flags. As such, WASM features are a common dependency.

There are a handful of limitations that cause the need to reimplement code entirely in this repository. Those are detailed below.

### Newtype pattern example

For example:
```rust
#[pyclass(name = "Mnemonic")]
pub struct PyMnemonic(pub Mnemonic);

#[pymethods]
impl PyMnemonic {
	#[getter]
    #[pyo3(name = "phrase")]
    pub fn phrase_string_py(&self) -> String {
        self.0.phrase().to_string()
    }
	
	...
}
```

### Enums

RK native enums cannot be exposed to Python as is. As such, wrapper enums for exposure to Python are defined, with conversion traits to/from the RK native enum.

There is a set of macros, for c-like and unit enums, that assist with definition and conversion trait implementions.

### Limitations

There are a handful of limitations to note.

- Private structs/methods/etc. in RK native. Cannot be leveraged by this repository, results in reimplementation here.
- RK native enums cannot be exposed as is. A Python interface compatible enum must be defined, with conversion traits to the corresponding RK native enum.
- RK native errors cannot be propped. Currently, `map_err()` is used extensively to convert to a generic Python exception. Future work will define explicit exceptions.

The Python package `kaspa` is built from the `kaspa-python` crate, which is located at `./python`. 
