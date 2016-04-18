
# `parse-generics-and-where` RFC Proof of Concept

This repository contains a proof-of-concept implementation for the `parse-generics-and-where` RFC.  It requires the `rustc` nightly from 2016-04-06.

`parse-generics-poc` contains the implementation of the `parse_generics!` and `parse_where!` macros.

`parse-macros` implements stable `macro_rules!` macros for destructuring the various Rust items by making use of the `parse_generics!` and `parse_where!` macros.  Note that these macros *have not* been updated for the specialisation RFC, or the restricted `pub` RFC.

## Examples and Tests

### `parse-generics-poc`

* `tests/derive_clone_copy.rs`: derives an implementation of `Clone` that uses `Copy`.
* `tests/derive_iterator.rs`: derives `Iterator` for newtype `struct`s.
* `tests/simple.rs`: asserts the output of the macros.
* `tests/wrap_fn.rs`: defines a macro to wrap a function with prologue/epilogue code.

### `parse-macros`

* `examples/derive_clone.rs`: stable `macro_rules!` re-implementation of the existing `Clone` derive attribute.
* `examples/derive_partial_ord.rs`: stable `macro_rules!` re-implementation of the existing `PartialOrd` derive attribute.
* `examples/reflect.rs`: stable macro to derive (partial and inefficient) runtime reflection structures.
* `examples/sandbox.rs`: dumps the parsed form of some random bits of code.
* `tests/derive_serialize.rs`: stable macro that derives serde's `Serialize` trait.
