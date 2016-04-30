#[doc(hidden)]
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
