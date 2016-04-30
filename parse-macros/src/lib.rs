/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides high-level macros for parsing various Rust constructs.

Specifically, these macros are concerned with taking Rust source constructs and rewriting them into a format which is more easily consumable by `macro_rules!` macros.

<style type="text/css">
.link-block { font-family: "Fira Sans"; }
.link-block > p { display: inline-block; }
.link-block > p > strong { font-weight: 500; margin-right: 1em; }
.link-block > ul { display: inline-block; padding: 0; list-style: none; }
.link-block > ul > li {
  font-size: 0.8em;
  background-color: #eee;
  border: 1px solid #ccc;
  padding: 0.3em;
  display: inline-block;
}
</style>
<span></span><div class="link-block">

**Links**

* [Latest Docs](https://danielkeep.github.io/rust-parse-generics/doc/parse_macros/index.html)
* [Repository](https://github.com/DanielKeep/rust-parse-generics)

<span></span></div>

## Table of Contents

- [`parse_enum!`](#parse_enum)
- [`parse_item!`](#parse_item)
- [`parse_struct!`](#parse_struct)

## `parse_enum!`

```ignore
macro_rules! parse_enum {
    (
        then $cb:ident!( $($cb_arg:tt)* ),
        $($body:tt)*
    ) => { ... };
}
```

Parses `$body` as an `enum`, invoking the macro `$cb` with the result.  The general form of the expansion is:

```ignore
$cb! {
    $($cb_arg)*
    enum {
        attrs: $attrs,
        vis: $vis,
        name: $name,
        generics: $generics,
        where: $where_,
        variants: $variants,
        num_variants: $num_variants,
    }
}
```

### Callback

`$cb_name` and `$cb_arg` specify the macro to invoke with the result of parsing.  Note that `$cb_arg` may be contained in *any* of `( .. )`, `[ .. ]`, or `{ .. }`.

### Fields

The expansion contains the following fields:

- `$attrs`: a `[ .. ]`-delimited list of attributes.  *e.g.*: `[ #[doc="Does a thing"] #[repr(u8)] ]`.

- `$vis`: a `( .. )`-delimited visibility annotation.  *e.g.*: `()`, `(pub)`.

- `$name`: the `enum`'s name as an identifier.  *e.g.*: `Option`.

- `$generics`: the `{ .. }`-delimited output of `parse_generics_shim!` for the `enum`, containing the `constr`, `params`, `ltimes`, and `tnames` fields:

    ```ignore
    generics: {
        constr: $constr,
        params: $params,
        ltimes: $ltimes,
        tnames: $tnames,
    }
    ```

    - `$constr`: a `[ .. ]`-delimited, comma-terminated list of generic constraints.  *e.g.* `['a, 'b: 'a, T, U: 'a + Copy,]`.

    - `$params`: a `[ .. ]`-delimited, comma-terminated list of generic parameter names.  *e.g.* `['a, 'b, T, U,]`.

    - `$ltimes`: a `[ .. ]`-delimited, comma-terminated list of generic lifetime parameters.  *e.g.* `['a, 'b,]`.

    - `$tnames`: a `[ .. ]`-delimited, comma-terminated list of generic type parameters.  *e.g.* `[T, U,]`.

- `$where_`: the `{ .. }`-delimited output of `parse_where_shim!` for the `enum`, containing the `preds` field:

    ```ignore
    where: {
        preds: $preds,
    }
    ```

    - `$preds`: a `[ .. ]`-delimited, comma-separated list of clause predicates.  *e.g.* `[ for<'a> T: Fn(&'a i32), ]`.

- `$variants`: a `[ .. ]`-delimited, comma-terminated list of variants (described below).

- `$num_variants`: the number of variants in the `enum`.  *e.g.* `2`.

Each variant has the following form:

```ignore
{
    ord: ($vord_index, $vord_ident),
    attrs: $vattrs,
    kind: $vkind,
    name: $vname,
    fields: $vfields,
    num_fields: $vnum_fields,
}
```

- `$vord_index`: the 0-based ordinal for this variant.  *e.g.* `1`.

- `$vord_ident`: an identifier guaranteed to be unique relative to other variants *for the same `enum`*.  Identifiers are *not* guaranteed to be unique between different `parse_enum!` invocations.  *e.g.* `_ord_01`.

- `$vattrs`: a `[ .. ]`-delimited list of attributes attached to the variant.  *e.g.* `[ #[doc="A variant unlike the rest."] ]`.

- `$vkind`: one of `unitary`, `tuple`, or `record`.

- `$vname`: the variant's name as an identifier.  *e.g.* `None`.

- `$vfields`: a `[ .. ]`-delimited, comma-terminated list of fields (described below).

- `$vnum_fields`: the number of fields in the variant.  *e.g.* `1`.

Variant fields have the following form:

```ignore
{
    ord: ($ford_index, $ford_ident),
    attrs: $fattrs,
    vis: $fvis,
    ty: $fty,

    // **NOTE**: only exists for *record* variant fields:
    name: $fname,
}
```

- `$ford_index`: the 0-based ordinal for this variant field.  *e.g.* `1`.

- `$ford_ident`: an identifier guaranteed to be unique relative to other fields *for the same variant*.  Identifiers are *not* guaranteed to be unique between different `parse_enum!` invocations, or between variants in the same invocation.  *e.g.* `_ord_01`.

- `$fattrs`: a `[ .. ]`-delimited list of attributes attached to the variant field.  *e.g.* `[ #[doc="A part of the whole."] ]`.

- `$fvis`: a `( .. )`-delimited visibility annotation.  *e.g.*: `()`, `(pub)`.

- `$fty`: the type of the variant field.

- `$fname`: the variant field's name as an identifier.  *e.g.* `part`.

### Example

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# #[macro_use] extern crate parse_macros;
# fn main() {
# assert_eq!( (
parse_enum! {
    then stringify!(output:),
    /// The `Option` type.
    pub enum Option<T> {
        /// No value.
        None,
        /// Some value `T`.
        Some(T),
        /// File could not be found.
        FileNotFound { path: PathBuf },
    }
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , r#"
    output:
    enum {
        attrs: [ #[doc=r"The `Option` type."] ],
        vis: (pub),
        name: Option,
        generics: {
            constr: [T,],
            params: [T,],
            ltimes: [],
            tnames: [T,],
        },
        where: {
            preds: [],
        },
        variants: [
            {
                ord: (0, _ord_00),
                attrs: [ #[doc=r"No value."] ],
                kind: unitary,
                name: None,
                fields: [],
                num_fields: 0,
            },
            {
                ord: (1, _ord_01),
                attrs: [ #[doc=r"Some value `T`."] ],
                kind: tuple,
                name: Some,
                fields: [
                    {
                        ord: (0, _ord_00),
                        attrs: [],
                        vis: (),
                        ty: T,
                    },
                ],
                num_fields: 1,
            },
            {
                ord: (2, _ord_02),
                attrs: [ #[doc=r"File could not be found."] ],
                kind: record,
                name: FileNotFound,
                fields: [
                    {
                        ord: (0, _ord_00),
                        attrs: [],
                        vis: (),
                        ty: PathBuf,
                        name: path,
                    },
                ],
                num_fields: 1,
            },
        ],
        num_variants: 3,
    }
# "#.replace(char::is_whitespace, "")); /*
)
# */ }
```

## `parse_item!`

```ignore
macro_rules! parse_item {
    (
        then $cb:ident!( $($cb_arg:tt)* ),
        $($body:tt)*
    ) => { ... };
}
```

Parses `$body` as an item, invoking the macro `$cb` with the result.  This forwards to the appropriate `parse_*!` macro, depending on what kind of item is in `$body`.

See [`parse_enum!`](#parse_enum), and [`parse_struct!`](#parse_struct) for more details.

## `parse_struct!`

```ignore
macro_rules! parse_struct {
    (
        then $cb:ident!( $($cb_arg:tt)* ),
        $($body:tt)*
    ) => { ... };
}
```

Parses `$body` as a `struct`, invoking the macro `$cb` with the result.  The general form of the expansion is:

```ignore
$cb! {
    $($cb_arg)*
    struct {
        attrs: $attrs,
        vis: $vis,
        name: $name,
        generics: $generics,
        where: $where_,
        kind: $kind,
        fields: $fields,
        num_fields: $num_fields,
    }
}
```

### Callback

`$cb_name` and `$cb_arg` specify the macro to invoke with the result of parsing.  Note that `$cb_arg` may be contained in *any* of `( .. )`, `[ .. ]`, or `{ .. }`.

### Fields

The expansion contains the following fields:

- `$attrs`: a `[ .. ]`-delimited list of attributes.  *e.g.*: `[ #[doc="Does a thing"] #[repr(C)] ]`.

- `$vis`: a `( .. )`-delimited visibility annotation.  *e.g.*: `()`, `(pub)`.

- `$name`: the `struct`'s name as an identifier.  *e.g.*: `Option`.

- `$generics`: the `{ .. }`-delimited output of `parse_generics_shim!` for the `struct`, containing the `constr`, `params`, `ltimes`, and `tnames` fields:

    ```ignore
    generics: {
        constr: $constr,
        params: $params,
        ltimes: $ltimes,
        tnames: $tnames,
    }
    ```

    - `$constr`: a `[ .. ]`-delimited, comma-terminated list of generic constraints.  *e.g.* `['a, 'b: 'a, T, U: 'a + Copy,]`.

    - `$params`: a `[ .. ]`-delimited, comma-terminated list of generic parameter names.  *e.g.* `['a, 'b, T, U,]`.

    - `$ltimes`: a `[ .. ]`-delimited, comma-terminated list of generic lifetime parameters.  *e.g.* `['a, 'b,]`.

    - `$tnames`: a `[ .. ]`-delimited, comma-terminated list of generic type parameters.  *e.g.* `[T, U,]`.

- `$where_`: the `{ .. }`-delimited output of `parse_where_shim!` for the `struct`, containing the `preds` field:

    ```ignore
    where: {
        preds: $preds,
    }
    ```

    - `$preds`: a `[ .. ]`-delimited, comma-separated list of clause predicates.  *e.g.* `[ for<'a> T: Fn(&'a i32), ]`.

- `$kind`: one of `unitary`, `tuple`, or `record`.  These correspond to the three kinds of `struct` definitions: `struct Unitary;`, `struct Tuple(..);` and `struct Record { .. }`.

- `$fields`: a `[ .. ]`-delimited, comma-terminated list of fields (described below).

- `$num_fields`: the number of fields in the `struct`.  *e.g.* `2`.

`struct` fields have the following form:

```ignore
{
    ord: ($ford_index, $ford_ident),
    attrs: $fattrs,
    vis: $fvis,
    ty: $fty,

    // **NOTE**: only exists for *record* `struct` fields:
    name: $fname,
}
```

- `$ford_index`: the 0-based ordinal for this `struct` field.  *e.g.* `1`.

- `$ford_ident`: an identifier guaranteed to be unique relative to other fields *for the same `struct`*.  Identifiers are *not* guaranteed to be unique between different `parse_struct!` invocations.  *e.g.* `_ord_01`.

- `$fattrs`: a `[ .. ]`-delimited list of attributes attached to the `struct` field.  *e.g.* `[ #[doc="The amount of green-ness."] ]`.

- `$fvis`: a `( .. )`-delimited visibility annotation.  *e.g.*: `()`, `(pub)`.

- `$fty`: the type of the `struct` field.

- `$fname`: the `struct` field's name as an identifier.  *e.g.* `green`.

### Example

```rust
# #![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
# #![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
# #[macro_use] extern crate parse_generics_shim;
# #[macro_use] extern crate parse_macros;
# fn main() {
# assert_eq!( (
parse_struct! {
    then stringify!(output:),
    /// Represents a colour.
    pub struct Rgb<Ch> {
        /// The degree of red-ness.
        r: Ch,
        /// How eco-friendly is this colour?
        g: Ch,
        /// Maybe it's blue, maybe it's not?
        b: Option<Ch>,
    }
}

// Expands to:
# /*
stringify!(
# */
# ).replace(char::is_whitespace, "") , r#"
    output:
    struct {
        attrs: [ #[doc=r"Represents a colour."] ],
        vis: (pub),
        name: Rgb,
        generics: {
            constr: [Ch,],
            params: [Ch,],
            ltimes: [],
            tnames: [Ch,],
        },
        where: {
            preds: [],
        },
        kind: record,
        fields: [
            {
                ord: (0, _ord_00),
                attrs: [ #[doc=r"The degree of red-ness."] ],
                vis: (),
                ty: Ch,
                name: r,
            },
            {
                ord: (1, _ord_01),
                attrs: [ #[doc=r"How eco-friendly is this colour?"] ],
                vis: (),
                ty: Ch,
                name: g,
            },
            {
                ord: (2, _ord_02),
                attrs: [ #[doc=r"Maybe it's blue, maybe it's not?"] ],
                vis: (),
                ty: Option<Ch>,
                name: b,
            },
        ],
        num_fields: 3,
    }
# "#.replace(char::is_whitespace, "")); /*
)
# */ }
```
*/
#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_use] mod parse_enum;
#[macro_use] mod parse_item;
#[macro_use] mod parse_macros_util;
#[macro_use] mod parse_struct;
