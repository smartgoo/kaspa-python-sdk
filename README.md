# Kaspa Python SDK

> [!CAUTION]
> **This repository is a proof of concept. Intended solely to assess feasibility of moving Kaspa Python SDK into its own repository.**
>
> **Do NOT use this repository.**
> 
> **The current `kaspa` Python package source can be found [here](https://github.com/aspectron/rusty-kaspa/tree/python).**

## Bindings Approach/Design
This project attempts to leverage native/existing Rusty-Kaspa source as much as possible. This is accomplished by defining Python-compatible code that wraps RK native. Generally, this repository tries to adhere to the following:

- Wrappers should perform only type conversion to/from RK native (to the extent possible).
- When logic is needed, RK native logic should be used (to the extent possible).

These principles work to varying degrees depending on the area of code, limitations, and Python interface requirements. In a "worst case" scenario, new/re-implementation is done in this repository.

All Python exposed structs and enums defined in this repository are prefixed with `Py` (e.g. `PyRpcClient`). These are then exposed to for use in Python without the prefix. Functions exposed to Python are prefixed with `py_` and are usable from Python without the `py_` prefix.

The [newtype pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) is used extensively for structs. Methods of the wrapped RK native struct are then leveraged as much as possible, typically by also wrapping the methods themselves for exposure to Python.

This repository relies on RK features gated behind WASM feature flags. As such, RK WASM features are a dependency.

There are a handful of limitations that cause the need for new or re-implementation in this repository. Those are detailed below.

The directory structure of this project approximately mirrors that of rusty-kaspa's workspace. This is likely to change but currently makes development easier.

## Limitations & Challenges

Limitations & challenges to note:

- Private code (structs, struct fields, methods, etc.) in RK native. Cannot be leveraged by this repository, results in new/re-implementation.
- RK native enums cannot be exposed as is. A Python interface compatible enum must be defined. To reduce boilerplate, some macros are included in this repository. 
- RK native errors cannot be propped. Currently, `map_err()` is used extensively to convert to a generic Python exception. Future work will define explicit exceptions.
- Python interface requires

Long term, to alleviate some challenges, there may be an opportunity to add "bindings primitives" to RK native. This is a separate topic though.
