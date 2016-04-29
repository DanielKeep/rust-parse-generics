/*!
This crate provides stable, partial implementations of the `parse_generics!` and `parse_where!` macros proposed in [RFC #1583].  These macros serve two purposes:

1. They allow crate authors to use the macros in a limited capacity whether or not the RFC is accepted.
2. They demonstrate to the Rust core team that there is demand for this functionality.
3. They provide a migration path from the partial implementation to the full one, assuming the RFC does get accepted.

Because these macros are implemented using `macro_rules!`, they have the following limitations:

- In general, only lifetimes `'a` through `'z` are accepted.
- Only a subset of the full output formats are supported.
- They are significantly less efficient, and consume a non-trivial amount of the recursion limit.

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

## `parse_where!`

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
    ; X
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , "
    output: {
        preds: [ 'a: 'b, T: 'a + Copy, for<'c,> U: Foo<'c>, ],
    },
    ; X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

```rust
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
    ; X
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
    ; X
# ".replace(char::is_whitespace, "")); /*
)
# */ }
```

The input does not *have* to start with a `where` clause:

```rust
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
*/

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
