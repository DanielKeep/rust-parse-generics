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
macro_rules! parse_enum {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub enum $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_enum! {
                @with_generics
                (($cb!$cb_arg), [$(#[$($attrs)*])*], (pub), $name),
            },
            $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* enum $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_enum! {
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
            { preds }, then parse_enum! {
                @with_where
                $prefix,
                $generics,
            },
            $($tail)*
        }
    };

    (
        @with_where
        $prefix:tt,
        $generics:tt,
        $preds:tt,
        { $($body:tt)* }
    ) => {
        parse_enum! {
            @parse_variants
            ($prefix, $generics, $preds),
            [],
            [],
            { $($body)*, },
            0, _ord_00
        }
    };

    (
        @parse_variants
        (
            (
                $cb:tt,
                $attrs:tt,
                $vis:tt,
                $name:ident
            ),
            $generics:tt,
            $preds:tt
        ),
        $variants:tt,
        $_attrs:tt,
        { $(,)* },
        $ord:tt, $_ord_ident:tt
    ) => {
        parse_macros_util! {
            @call $cb,
            enum {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                variants: $variants,
                num_variants: $ord,
            }
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        $variants:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_enum! {
            @parse_variants
            $prefix2,
            $variants,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord, $ord_ident
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        [$($variants:tt)*],
        $attrs:tt,
        { $vname:ident, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        kind: unitary,
                        name: $vname,
                        fields: [],
                        num_fields: 0,
                    },
                ],
                [],
                { $($tail)* },
            }),
            $ord
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        $variants:tt,
        $attrs:tt,
        { $vname:ident($($body:tt)*), $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_enum! {
            @parse_tuple_fields
            ($prefix2, $variants, $attrs, $vname, ($ord, $ord_ident), { $($tail)* }),
            [],
            [],
            ($($body)*,),
            0, _ord_00
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        $variants:tt,
        $attrs:tt,
        { $vname:ident { $($body:tt)* }, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_enum! {
            @parse_record_fields
            ($prefix2, $variants, $attrs, $vname, ($ord, $ord_ident), { $($tail)* }),
            [],
            [],
            { $($body)*, },
            0, _ord_00
        }
    };

    (
        @parse_tuple_fields
        (
            $prefix2:tt,
            [$($variants:tt)*],
            $attrs:tt,
            $vname:ident,
            ($ord:tt, $ord_ident:tt),
            $tail:tt
        ),
        $fields:tt,
        $_fattrs:tt,
        ( $(,)* ),
        $ford:tt, $ford_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        kind: tuple,
                        name: $vname,
                        fields: $fields,
                        num_fields: $ford,
                    },
                ],
                [],
                $tail,
            }),
            $ord
        }
    };

    (
        @parse_tuple_fields
        $prefix3:tt,
        $fields:tt,
        [$($attrs:tt)*],
        (#[$($attr:tt)*] $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_enum! {
            @parse_tuple_fields
            $prefix3,
            $fields,
            [$($attrs)* #[$($attr)*]],
            ($($tail)*),
            $ord, $ord_ident:tt
        }
    };

    (
        @parse_tuple_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        (pub $fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_tuple_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (pub),
                        ty: $fty,
                    },
                ],
                [],
                ($($tail)*),
            }),
            $ord
        }
    };

    (
        @parse_tuple_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        ($fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_tuple_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (),
                        ty: $fty,
                    },
                ],
                [],
                ($($tail)*),
            }),
            $ord
        }
    };

    (
        @parse_record_fields
        (
            $prefix2:tt,
            [$($variants:tt)*],
            $attrs:tt,
            $vname:ident,
            ($ord:tt, $ord_ident:tt),
            $tail:tt
        ),
        $fields:tt,
        $_fattrs:tt,
        { $(,)* },
        $ford:tt, $ford_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        kind: record,
                        name: $vname,
                        fields: $fields,
                        num_fields: $ford,
                    },
                ],
                [],
                $tail,
            }),
            $ord
        }
    };

    (
        @parse_record_fields
        $prefix3:tt,
        $fields:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_enum! {
            @parse_record_fields
            $prefix3,
            $fields,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord, $ord_ident
        }
    };

    (
        @parse_record_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { pub $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_record_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (pub),
                        ty: $fty,
                        name: $fname,
                    },
                ],
                [],
                { $($tail)* },
            }),
            $ord
        }
    };

    (
        @parse_record_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_enum! {
                @parse_record_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (),
                        ty: $fty,
                        name: $fname,
                    },
                ],
                [],
                { $($tail)* },
            }),
            $ord
        }
    };
}
