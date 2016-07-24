/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[doc(hidden)]
#[macro_export]
macro_rules! parse_type_item {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub type $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_type_item! {
                @with_generics
                (($cb!$cb_arg), [$(#[$($attrs)*])*], (pub), $name),
            },
            $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* type $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_type_item! {
                @with_generics
                (($cb!$cb_arg), [$(#[$($attrs)*])*], (), $name),
            },
            $($tail)*
        }
    };

    (
        @with_generics
        $prefix:tt,
        $generics:tt,
        $($tail:tt)*
    ) => {
        parse_where_shim! {
            { preds }, then parse_type_item! {
                @with_where
                $prefix, $generics,
            },
            $($tail)*
        }
    };

    (
        @with_where
        ($cb:tt, $attrs:tt, $vis:tt, $name:ident),
        $generics:tt,
        $preds:tt,
        = $t:ty;
    ) => {
        parse_macros_util! {
            @call $cb,
            type {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                type: $t,
            }
        }
    };
}
