/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides stable, partial implementations of the `parse_generics!` and `parse_where!` macros proposed in [RFC #1583].  These macros serve two purposes:

1. They allow crate authors to use the macros in a limited capacity whether or not the RFC is accepted.
2. They demonstrate to the Rust core team that there is demand for this functionality.
3. They provide a migration path from the partial implementation to the full one, assuming the RFC does get accepted.

Because these macros are implemented using `macro_rules!`, they have the following limitations:

- In general, only lifetimes `'a` through `'z` are accepted.
- Only a subset of the full output formats are supported.
- They are significantly less efficient, and consume a non-trivial amount of the recursion limit.

# Table of Contents

- [`parse_generics_shim!`](#parse_generics_shim)
- [`parse_where_shim!`](#parse_where_shim)
- [Supporting `parse-generics-poc`](#supporting-parse-generics-poc)

[RFC #1583]: https://github.com/rust-lang/rfcs/pull/1583

## `parse_generics_shim!`

```ignore
macro_rules! parse_generics_shim {
    (
        { $($fields:ident),+ },
        then $callback_name:ident ! ( $($callback_args:tt)* ),
        $($code:tt)*
    ) => { ... };
}
```

Parses a generic parameter list (if present) from the start of `$($code:tt)*`, expanding to the parsed information plus the unconsumed tokens *after* the parameter list.  The general form of the expansion is:

```ignore
$callback_name! {
    $($callback_args)*
    {
        $(
            $fields: [ .. ],
        )+
    },
    $($tail)*
}
```

### Callback

`$callback_name` and `$callback_args` specify the macro to invoke with the result of parsing.  Note that `$callback_args` may be contained in *any* of `( .. )`, `[ .. ]`, or `{ .. }`.

### Fields

`$fields` indicates which pieces of information you want in the expansion.  The available fields are:

- `constr` - comma-terminated list of generic parameters plus their constraints.
- `params` - comma-terminated list of generic parameter names (both lifetimes and types).
- `ltimes` - comma-terminated list of generic lifetime names.
- `tnames` - comma-terminated list of generic type names.

The shim *only* supports the following combinations:

- `{ constr, params, ltimes, tnames }`
- `{ constr }`
- `{ .. }`

The fields will appear in the output in the same order they appear in the input.  One special case is `{ .. }` which causes *all* fields to be emitted, followed by a literal `..` token.

**Warning**: there is explicitly *no* guarantee that the list of fields will stay the same over time.  As such, it is **strongly** recommended that you never directly match the `..` token after the fields.  Instead, you should use the following construct:

```ignore
macro_rules! match_output {
    (
        {
            // Match the fields you care about.
            constr: $constr:tt,
            params: [ $($params:tt,)* ],

            // Ignore the rest; *never* explicitly match `..`!
            $($_fields:tt)*
        },

        $($tail:tt)*
    ) => { ... };
}
```

### Code

`$code` is the actual source code to be parsed.  If it starts with `<`, the macro will parse a generic parameter list.  If it *does not* start with `<`, the macro will proceed as though the input started with an empty generic parameter list (*i.e.* `<>`).

### Examples

The following show how the various invocation forms affect the output:

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_generics_shim! {
    { constr, params, ltimes, tnames },
    then stringify!(output:),
    <'a, T, U: 'a + Copy> X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        constr: [ 'a, T, U: 'a + Copy, ],
        params: [ 'a, T, U, ],
        ltimes: [ 'a, ],
        tnames: [ T, U, ],
    },
    X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_generics_shim! {
    { constr },
    then stringify!(output:),
    <'a, T, U: 'a + Copy> X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        constr: [ 'a, T, U: 'a + Copy, ],
    },
    X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_generics_shim! {
    { .. },
    then stringify!(output:),
    <'a, T, U: 'a + Copy> X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        constr: [ 'a, T, U: 'a + Copy, ],
        params: [ 'a, T, U, ],
        ltimes: [ 'a, ],
        tnames: [ T, U, ],
        ..
    },
    X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

The input does not *have* to start with a generic parameter list.  Note that both of the invocations below expand to the same result:

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_generics_shim! {
    { constr, params, ltimes, tnames },
    then stringify!(output:),
    <> X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        constr: [],
        params: [],
        ltimes: [],
        tnames: [],
    },
    X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_generics_shim! {
    { constr, params, ltimes, tnames },
    then stringify!(output:),
    X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        constr: [],
        params: [],
        ltimes: [],
        tnames: [],
    },
    X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

## `parse_where_shim!`

```ignore
macro_rules! parse_where_shim {
    (
        { $($fields:ident),+ },
        then $callback_name:ident ! ( $($callback_args:tt)* ),
        $($code:tt)*
    ) => { ... };
}
```

Parses a `where` clause (if present) from the start of `$($code:tt)*`, expanding to the parsed information plus the unconsumed tokens *after* the clause.  The general form of the expansion is:

```ignore
$callback_name! {
    $($callback_args)*
    {
        $(
            $fields: [ .. ],
        )+
    },
    $($tail)*
}
```

### Callback

`$callback_name` and `$callback_args` specify the macro to invoke with the result of parsing.  Note that `$callback_args` may be contained in *any* of `( .. )`, `[ .. ]`, or `{ .. }`.

### Fields

`$fields` indicates which pieces of information you want in the expansion.  The available fields are:

- `preds` - comma-terminated list of predicates.

The shim *only* supports the following combinations:

- `{ preds }`
- `{ .. }`

The fields will appear in the output in the same order they appear in the input.  One special case is `{ .. }` which causes *all* fields to be emitted, followed by a literal `..` token.

**Warning**: there is explicitly *no* guarantee that the list of fields will stay the same over time.  As such, it is **strongly** recommended that you never directly match the `..` token after the fields.  Instead, you should use the following construct:

```ignore
macro_rules! match_output {
    (
        {
            // Match the fields you care about.
            preds: [ $($preds:tt)* ],

            // Ignore the rest; *never* explicitly match `..`!
            $($_fields:tt)*
        },

        $($tail:tt)*
    ) => { ... };
}
```

### Code

`$code` is the actual source code to be parsed.  If it starts with `where`, the macro will parse a `where` clause, stopping when it encounters any of the following: `;`, `{`, or `=`.  If it *does not* start with `where`, the macro will expand with an empty predicate list.

### Examples

The following show how the various invocation forms affect the output:

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_where_shim! {
    { preds },
    then stringify!(output:),
    where
        'a: 'b,
        T: 'a + Copy,
        for<'c> U: Foo<'c>,
    { struct fields... }
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        preds: [ 'a: 'b, T: 'a + Copy, for<'c,> U: Foo<'c>, ],
    },
    { struct fields... }
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_where_shim! {
    { .. },
    then stringify!(output:),
    where
        'a: 'b,
        T: 'a + Copy,
        for<'c> U: Foo<'c>,
    { struct fields... }
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        preds: [ 'a: 'b, T: 'a + Copy, for<'c,> U: Foo<'c>, ],
        ..
    },
    { struct fields... }
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

The input does not *have* to start with a `where` clause:

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# fn main() {
# assert_eq!( (
parse_where_shim! {
    { preds },
    then stringify!(output:),
    ; X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        preds: [],
    },
    ; X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

## Supporting `parse-generics-poc`

If you wish to enable support for the full `parse_generics!` and `parse_where!` macros *instead* of the shim implementations, you need to create a feature which will enable the POC macros.  This can be done by adding the following to your `Cargo.toml` manifest:

```toml
[features]
use-parse-generics-poc = [
    "parse-generics-poc",
    "parse-generics-shim/use-parse-generics-poc",
]

[dependencies]
parse-generics-poc = { version = "0.1.0", optional = true }
parse-generics-shim = "0.1.0"
```

You should also pass the following information on to your users (or direct them here):

### Using `parse-generics-poc`

To activate `parse-generics-poc` support in the `whizzo` crate, you must:

1. Enable the `use-parse-generics-poc` feature in your crate.  This is best done using a forwarding feature in your `Cargo.toml` so that your code is not unavoidably dependant on a nightly compiler:

    ```toml
    [features]
    use-parse-generics-poc = [
        "parse-generics-poc",
        "whizzo/use-parse-generics-poc"
    ]
    ```

2. Depend on both the relevant macro crates.  Because compiler plugins and macros can't be re-exported, this is sadly unavoidable.

    ```toml
    [dependencies]
    parse-generics-poc = { version = "0.1.0", optional = true }
    parse-generics-shim = "0.1.0"
    ```

3. You have to use a nightly compiler compatible with `parse-generics-poc`.  The documentation for `parse-generics-poc` should specify *which* nightly it is known to be compatible with.  If you are using `rustup`, you can configure your crate to use the appropriate compiler using the following (replacing the date shown with the one listed in the `parse-generics-poc` documentation):

    ```sh
    rustup override add nightly-2016-04-06
    ```

4. You must add the following attributes to the top of your crate's root module:

    ```rust
    #![cfg_attr(feature="parse-generics-poc", feature(plugin))]
    #![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
    #[macro_use] extern crate parse_generics_shim;
    ```
*/
#![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]

#[cfg(not(feature="use-parse-generics-poc"))]
#[doc(hidden)]
#[macro_export]
macro_rules! parse_generics_shim_util {
    (
        @callback
        ($cb_name:ident ! ($($cb_arg:tt)*)),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! [$($cb_arg:tt)*]),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! {$($cb_arg:tt)*}),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };
}

mod parse_constr;
mod parse_generics_shim;
mod parse_where_shim;
