#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_export]
macro_rules! parse_item {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub enum $($tail:tt)*
    ) => {
        parse_enum! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* pub enum $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* enum $($tail:tt)*
    ) => {
        parse_enum! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* enum $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub struct $($tail:tt)*
    ) => {
        parse_struct! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* pub struct $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* struct $($tail:tt)*
    ) => {
        parse_struct! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* struct $($tail)*
        }
    };
}

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
            [$($attrs)* $($attr)*],
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
                        fields: (),
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

#[macro_export]
macro_rules! parse_struct {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub struct $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_struct! {
                @with_generics
                (($cb!$cb_arg), [$(#[$($attrs)*])*], (pub), $name),
            },
            $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* struct $name:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr, params, ltimes, tnames },
            then parse_struct! {
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
        ($($body:tt)*) $($tail:tt)*
    ) => {
        parse_where_shim! {
            { preds }, then parse_struct! {
                @with_where
                $prefix, $generics,
            },
            $($tail)* ($($body)*)
        }
    };

    (
        @with_generics
        $prefix:tt,
        $generics:tt,
        $($tail:tt)*
    ) => {
        parse_where_shim! {
            { preds }, then parse_struct! {
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
        ;
    ) => {
        parse_macros_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                kind: unitary,
                fields: [],
                num_fields: 0,
            }
        }
    };

    (
        @with_where
        $prefix:tt,
        $generics:tt,
        $preds:tt,
        ; ($($body:tt)*)
    ) => {
        parse_struct! {
            @parse_fields
            ($prefix, $generics, $preds),
            [],
            [],
            ($($body)*,),
            0, _ord_00
        }
    };

    (
        @with_where
        $prefix:tt,
        $generics:tt,
        $preds:tt,
        {$($body:tt)*}
    ) => {
        parse_struct! {
            @parse_fields
            ($prefix, $generics, $preds),
            [],
            [],
            {$($body)*,},
            0, _ord_00
        }
    };

    (
        @parse_fields
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
        $fields:tt,
        $_attrs:tt,
        ($(,)*),
        $ord:tt, $_ord_ident:tt
    ) => {
        parse_macros_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                kind: tuple,
                fields: $fields,
                num_fields: $ord,
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        (#[$($attr:tt)*] $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            ($($tail)*),
            $ord, $ord_ident
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        (pub $fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_struct! {
                @parse_fields
                $prefix2,
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
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        ($fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_struct! {
                @parse_fields
                $prefix2,
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
        @parse_fields
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
        $fields:tt,
        $_attrs:tt,
        { $(,)* },
        $ord:tt, $_ord_ident:tt
    ) => {
        parse_macros_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                kind: record,
                fields: $fields,
                num_fields: $ord,
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord, $ord_ident
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { pub $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (pub),
                        name: $fname,
                        ty: $fty,
                    },
                ],
                [],
                {$($tail)*},
            }),
            $ord
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        parse_macros_util! {
            @inc_ord_ident
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (),
                        name: $fname,
                        ty: $fty,
                    },
                ],
                [],
                {$($tail)*},
            }),
            $ord
        }
    };
}

#[macro_export]
macro_rules! parse_macros_util {
    (
        @call
        ($cb:ident!($($cb_arg:tt)*)),
        $($output:tt)*
    ) => {
        $cb!(
            $($cb_arg)*
            $($output)*
        )
    };

    (
        @call
        ($cb:ident!{$($cb_arg:tt)*}),
        $($output:tt)*
    ) => {
        $cb! {
            $($cb_arg)*
            $($output)*
        }
    };

    (@inc_ord_ident $cb:tt,  0) => { parse_macros_util!{ @call $cb,  1, _ord_01 } };
    (@inc_ord_ident $cb:tt,  1) => { parse_macros_util!{ @call $cb,  2, _ord_02 } };
    (@inc_ord_ident $cb:tt,  2) => { parse_macros_util!{ @call $cb,  3, _ord_03 } };
    (@inc_ord_ident $cb:tt,  3) => { parse_macros_util!{ @call $cb,  4, _ord_04 } };
    (@inc_ord_ident $cb:tt,  4) => { parse_macros_util!{ @call $cb,  5, _ord_05 } };
    (@inc_ord_ident $cb:tt,  5) => { parse_macros_util!{ @call $cb,  6, _ord_06 } };
    (@inc_ord_ident $cb:tt,  6) => { parse_macros_util!{ @call $cb,  7, _ord_07 } };
    (@inc_ord_ident $cb:tt,  7) => { parse_macros_util!{ @call $cb,  8, _ord_08 } };
    (@inc_ord_ident $cb:tt,  8) => { parse_macros_util!{ @call $cb,  9, _ord_09 } };
    (@inc_ord_ident $cb:tt,  9) => { parse_macros_util!{ @call $cb, 10, _ord_10 } };
    (@inc_ord_ident $cb:tt, 10) => { parse_macros_util!{ @call $cb, 11, _ord_11 } };
    (@inc_ord_ident $cb:tt, 11) => { parse_macros_util!{ @call $cb, 12, _ord_12 } };
    (@inc_ord_ident $cb:tt, 12) => { parse_macros_util!{ @call $cb, 13, _ord_13 } };
    (@inc_ord_ident $cb:tt, 13) => { parse_macros_util!{ @call $cb, 14, _ord_14 } };
    (@inc_ord_ident $cb:tt, 14) => { parse_macros_util!{ @call $cb, 15, _ord_15 } };
    (@inc_ord_ident $cb:tt, 15) => { parse_macros_util!{ @call $cb, 16, _ord_16 } };
    (@inc_ord_ident $cb:tt, 16) => { parse_macros_util!{ @call $cb, 17, _ord_17 } };
    (@inc_ord_ident $cb:tt, 17) => { parse_macros_util!{ @call $cb, 18, _ord_18 } };
    (@inc_ord_ident $cb:tt, 18) => { parse_macros_util!{ @call $cb, 19, _ord_19 } };
    (@inc_ord_ident $cb:tt, 19) => { parse_macros_util!{ @call $cb, 20, _ord_20 } };
    (@inc_ord_ident $cb:tt, 20) => { parse_macros_util!{ @call $cb, 21, _ord_21 } };
    (@inc_ord_ident $cb:tt, 21) => { parse_macros_util!{ @call $cb, 22, _ord_22 } };
    (@inc_ord_ident $cb:tt, 22) => { parse_macros_util!{ @call $cb, 23, _ord_23 } };
    (@inc_ord_ident $cb:tt, 23) => { parse_macros_util!{ @call $cb, 24, _ord_24 } };
    (@inc_ord_ident $cb:tt, 24) => { parse_macros_util!{ @call $cb, 25, _ord_25 } };
    (@inc_ord_ident $cb:tt, 25) => { parse_macros_util!{ @call $cb, 26, _ord_26 } };
    (@inc_ord_ident $cb:tt, 26) => { parse_macros_util!{ @call $cb, 27, _ord_27 } };
    (@inc_ord_ident $cb:tt, 27) => { parse_macros_util!{ @call $cb, 28, _ord_28 } };
    (@inc_ord_ident $cb:tt, 28) => { parse_macros_util!{ @call $cb, 29, _ord_29 } };
    (@inc_ord_ident $cb:tt, 29) => { parse_macros_util!{ @call $cb, 30, _ord_30 } };
    (@inc_ord_ident $cb:tt, 30) => { parse_macros_util!{ @call $cb, 31, _ord_31 } };
    (@inc_ord_ident $cb:tt, 31) => { parse_macros_util!{ @call $cb, 32, _ord_32 } };
}
