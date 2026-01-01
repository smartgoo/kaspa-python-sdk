# Kaspa Python SDK

> [!CAUTION]
> **This repository is a proof of concept. Intended solely to assess feasibility of moving Kaspa Python SDK into its own repository.**
>
> **Do NOT use this repository.**
> 
> **The current `kaspa` Python package source can be found [here](https://github.com/aspectron/rusty-kaspa/tree/python).**

---

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

---

## Limitations & Challenges

Limitations & challenges to note:

- Private code (structs, struct fields, methods, etc.) in RK native. Cannot be leveraged by this repository, results in new/re-implementation.
- RK native enums cannot be exposed as is. A Python interface compatible enum must be defined. To reduce boilerplate, some macros are included in this repository. 
- RK native errors cannot be propped. Currently, `map_err()` is used extensively to convert to a generic Python exception. Future work will define explicit exceptions.
- Python interface requires

Long term, to alleviate some challenges, there may be an opportunity to add "bindings primitives" to RK native. This is a separate topic though.

---

## Rust <-> PyO3 <-> CPython Overview

The following is a brief overview of core concepts required to understand the Rust/CPython bridge provided by PyO3.

### Python Memory Model
In CPython, all data is stored on a private heap managed by CPython. The lifetime of all objects on this heap is managed via reference counting.

All data is stored on the heap in a `PyObject*` container class. This includes primitives (integers, strings, etc.) as well as custom classes and so on.

The `PyObject*` container has a field `ob_refcnt` which tracks the reference count for the given object. As references are created, this field is incremented using CPython's `Py_INCREF()` function. As references are dropped, it is decremented using `Py_DECCREF()`.

When the reference count of data on the heap (again, inside of a `PyObject*`) reaches 0, the memory is deallocated. 

### CPython & The C-API

CPython is the original, de facto, and most-widely used Python implementation. CPython is not *Python the language*. 

CPython is effectively the runtime for the *Python the language*:
- Compiles Python code into bytecode.
- Runs the bytecode on its Python Virtual Machine.
- Provides functionality like the standard library, built in functions, etc.

The Python C-API exposes (a huge set of) functionality for integrating with CPython. Including APIs for object creation on the Python heap, reference count management, executing Python functions, and so on. This allows CPython to load external libraries, called extension modules, making them usable from your Python program. Extension modules can be written in any language that can interact with Python's C-API.

### CPython C-API Concepts & Terminology
- **Owned reference**: A pointer to data on the Python heap. The holder contributes responsibility to the objects lifetime by incrementing (`Py_INCREF()`) and decrementing (`Py_DECCREF()`) the reference count of the object on the Python heap.
- **Borrowed reference**: A pointer to data on the Python heap. The **holder does not** increment/decrement the refcount and must not outlive the actual owner (risking a dangling pointer if the object is destroyed).

### PyO3

PyO3 is a Rust library that provides deep integration with Python's C-API. This allows the creation of Python extension modules from the Rust source code.

PyO3 provides low-level (one-to-one) bindings to Python's C-API via Rust's FFI functionality.

PyO3 also provides abstractions on top of these lower-level bindings that help avoid common pitfalls of working with Python. These abstractions also bridge the Rust and Python memory models, providing an ergonomic and safe solution.

#### PyO3's `FromPyObject` Trait

First, it's important to understand [PyO3's `FromPyObject` trait](https://pyo3.rs/v0.27.2/conversions/traits#extract-and-the-frompyobject-trait).

All types exposed to Python (primitives and custom) implement the PyO3 `FromPyObject` trait. This trait provides the `extract()` method which is used to bridge Python heap data to Rust.

`FromPyObject` is implemented out of the box for primitive types, and automatically for structs/enums decorated with `#[pyclass]` attribute.

The standard `FromPyObject` impl performs a clone under the hood, requiring the `#[pyclass]` decorated type to implement the Clone trait. This results in cloning data from the Python heap into Rust-managed memory.

`FromPyObject` can also be manually implemented to extend functionality and/or change default behavior.

#### Primitive Type  Function Parameters
TODO

#### Custom Type function Parameters 

Custom types are structs or enums decorated with `#[pyclass]` attribute.

PyO3 offers a few methods for specifying parameters of custom types in Python-exposed functions. Each method comes with it's own underlying mechanics. The following is a brief summary covering approaches used in this project.

**`T` (`#[pyclass]` with standard `FromPyObject` impl):**

Use when: Rust ownership of function args is needed.

- An owned Rust value.
- Requires Clone trait on the `[pyclass]` type.
- Clones data from the Python heap into Rust-managed memory.
- Standard Rust ownership applies from there.
- The original object on the Python heap remains unchanged.
- The CPython managed reference count remains unchanged.

**`Py<T>`**:

Use when: storing Python objects (references) long-term in Rust.

- A GIL-independent Rust-owned reference to a value on the Python heap.
- Not bound to Python GIL, but can be bound when Python interpreter is needed to perform some action.
- A Rust-owned reference to data on the Python heap.
- The CPython managed reference count increments on creation and decrements on drop.

**`Bound<'py, T>`:**

Use when: Rust ownership (of a reference) with GIL is required (e.g. creating new Python objects from Rust, etc).

- A GIL-bound reference to a value on the Python heap.
- A Rust-owned reference to data on the Python heap.
- The CPython managed reference count increments on creation and decrements on drop.


**`&Bound<'py, T>`:**

Use when: reading or mutating without taking ownership.

- A GIL-bound borrowed reference to a value on the Python heap.
- "Borrowed reference" is a CPython concept, a pointer to a `PyObject*` on the Python heap, without incrementing/decrementing refcount. CPython guarantees the pointer is valid as long as some owning reference keeps the object alive.
- The CPython managed reference count remains unchanged.


| Type                  | Category                  | Lifetime / GIL Binding              | Ownership / Reference Counting                  | Access to Embedded Rust Data (for `#[pyclass]`)                  | Common Use Cases                                                                 | Performance / Notes                          |
|-----------------------|---------------------------|------------------------------------|-------------------------------------------------|-----------------------------------------------------------------|----------------------------------------------------------------------------------|----------------------------------------------|
| `T` | Owned Rust value | No lifetime (static-like). Completely outside of GIL | Owns a **cloned** copy in Rust memory| Direct access (`obj.field`)                                    | Function args when you want an owned copy (e.g., to store or move). Requires `T: Clone`. | Involves cloning the Rust data; avoid for large structs. Blanket `FromPyObject` clones by default. |
| `Py<T>` | GIL-independent owned reference | No `'py` lifetime | Owns a strong Python reference (refcount +1)   | Via `.bind(py).borrow()` or similar (need GIL)                 | Storing Python objects long-term (e.g., in `#[pyclass]` fields, across GIL releases, or threads). | GIL-independent; safe to hold without GIL, but most operations require reacquiring GIL. Convert from `Bound` via `.unbind()`. |
| `Bound<'py, T>`       | GIL-bound owned reference | Tied to `'py` (proof GIL is held) | Owns a strong Python reference (refcount +1)   | Via `.borrow()` / `.borrow_mut()`                              | Temporary ownership within a GIL-held scope (e.g., returning new objects, intermediate values). | Preferred for most operations; efficient Python API calls. |
| `&Bound<'py, T>`      | Borrowed reference        | Tied to `'py` (proof GIL is held) | Borrows (no refcount change)                   | Via `.borrow()` (immutable) or `.borrow_mut()` (mutable, with runtime checks) | Function arguments (most common); reading/mutating without taking ownership. | Zero-cost borrow; idiomatic for args. |
