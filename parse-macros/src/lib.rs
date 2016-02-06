#![feature(plugin)]
#![plugin(parse_generics_poc)]

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
        parse_generics! {
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
        parse_generics! {
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
        parse_where! {
            then parse_enum! {
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
            0
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
        $ord:tt
    ) => {
        parse_util! {
            @call $cb,
            enum {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $preds,
                variants: $variants,
                num_variants: $ord,
                ...
            }
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        $variants:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt
    ) => {
        parse_where! {
            @parse_variants
            $prefix2,
            $variants,
            [$($attrs)* $($attr)*],
            { $($tail)* },
            $ord
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        [$($variants:tt)*],
        $attrs:tt,
        { $vname:ident, $($tail:tt)* },
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: $ord,
                        attrs: $attrs,
                        kind: unitary,
                        name: $vname,
                        fields: (),
                        num_fields: 0,
                        ...
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
        $ord:tt
    ) => {
        parse_enum! {
            @parse_tuple_fields
            ($prefix2, $variants, $attrs, $vname, $ord, { $($tail)* }),
            [],
            [],
            ($($body)*,),
            0
        }
    };

    (
        @parse_variants
        $prefix2:tt,
        $variants:tt,
        $attrs:tt,
        { $vname:ident { $($body:tt)* }, $($tail:tt)* },
        $ord:tt
    ) => {
        parse_enum! {
            @parse_record_fields
            ($prefix2, $variants, $attrs, $vname, $ord, { $($tail)* }),
            [],
            [],
            { $($body)*, },
            0
        }
    };

    (
        @parse_tuple_fields
        (
            $prefix2:tt,
            [$($variants:tt)*],
            $attrs:tt,
            $vname:ident,
            $ord:tt,
            $tail:tt
        ),
        $fields:tt,
        $_fattrs:tt,
        ( $(,)* ),
        $ford:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: $ord,
                        attrs: $attrs,
                        kind: tuple,
                        name: $vname,
                        fields: $fields,
                        num_fields: $ford,
                        ...
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
        $ord:tt
    ) => {
        parse_enum! {
            @parse_tuple_fields
            $prefix3,
            $fields,
            [$($attrs)* #[$($attr)*]],
            ($($tail)*),
            $ord
        }
    };

    (
        @parse_tuple_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        (pub $fty:ty, $($tail:tt)*),
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_tuple_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_tuple_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
            $ord:tt,
            $tail:tt
        ),
        $fields:tt,
        $_fattrs:tt,
        { $(,)* },
        $ford:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_variants
                $prefix2,
                [
                    $($variants)*
                    {
                        ord: $ord,
                        attrs: $attrs,
                        kind: record,
                        name: $vname,
                        fields: $fields,
                        num_fields: $ford,
                        ...
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
        $ord:tt
    ) => {
        parse_enum! {
            @parse_record_fields
            $prefix3,
            $fields,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord
        }
    };

    (
        @parse_record_fields
        $prefix3:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { pub $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_record_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_enum! {
                @parse_record_fields
                $prefix3,
                [
                    $($fields)*
                    {
                        ord: $ord,
                        attrs: $attrs,
                        vis: (),
                        ty: $fty,
                        name: $fname,
                    },
                ],
                [],
                ($($tail)*),
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
        parse_generics! {
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
        parse_generics! {
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
        parse_where! {
            then parse_struct! {
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
        parse_where! {
            then parse_struct! {
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
        parse_util! {
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
                ...
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
            0
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
            0
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
        $ord:tt
    ) => {
        parse_util! {
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
                ...
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        (#[$($attr:tt)*] $($tail:tt)*),
        $ord:tt
    ) => {
        parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            ($($tail)*),
            $ord
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        (pub $fty:ty, $($tail:tt)*),
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
        $ord:tt
    ) => {
        parse_util! {
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
                ...
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt
    ) => {
        parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { pub $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
        $ord:tt
    ) => {
        parse_util! {
            @inc_ord
            (parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: $ord,
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
macro_rules! parse_util {
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

    (@inc_ord $cb:tt,  0) => { parse_util!{ @call $cb,  1 } };
    (@inc_ord $cb:tt,  1) => { parse_util!{ @call $cb,  2 } };
    (@inc_ord $cb:tt,  2) => { parse_util!{ @call $cb,  3 } };
    (@inc_ord $cb:tt,  3) => { parse_util!{ @call $cb,  4 } };
    (@inc_ord $cb:tt,  4) => { parse_util!{ @call $cb,  5 } };
    (@inc_ord $cb:tt,  5) => { parse_util!{ @call $cb,  6 } };
    (@inc_ord $cb:tt,  6) => { parse_util!{ @call $cb,  7 } };
    (@inc_ord $cb:tt,  7) => { parse_util!{ @call $cb,  8 } };
    (@inc_ord $cb:tt,  8) => { parse_util!{ @call $cb,  9 } };
    (@inc_ord $cb:tt,  9) => { parse_util!{ @call $cb, 10 } };
    (@inc_ord $cb:tt, 10) => { parse_util!{ @call $cb, 11 } };
    (@inc_ord $cb:tt, 11) => { parse_util!{ @call $cb, 12 } };
    (@inc_ord $cb:tt, 12) => { parse_util!{ @call $cb, 13 } };
    (@inc_ord $cb:tt, 13) => { parse_util!{ @call $cb, 14 } };
    (@inc_ord $cb:tt, 14) => { parse_util!{ @call $cb, 15 } };
    (@inc_ord $cb:tt, 15) => { parse_util!{ @call $cb, 16 } };
    (@inc_ord $cb:tt, 16) => { parse_util!{ @call $cb, 17 } };
    (@inc_ord $cb:tt, 17) => { parse_util!{ @call $cb, 18 } };
    (@inc_ord $cb:tt, 18) => { parse_util!{ @call $cb, 19 } };
    (@inc_ord $cb:tt, 19) => { parse_util!{ @call $cb, 20 } };
    (@inc_ord $cb:tt, 20) => { parse_util!{ @call $cb, 21 } };
    (@inc_ord $cb:tt, 21) => { parse_util!{ @call $cb, 22 } };
    (@inc_ord $cb:tt, 22) => { parse_util!{ @call $cb, 23 } };
    (@inc_ord $cb:tt, 23) => { parse_util!{ @call $cb, 24 } };
    (@inc_ord $cb:tt, 24) => { parse_util!{ @call $cb, 25 } };
    (@inc_ord $cb:tt, 25) => { parse_util!{ @call $cb, 26 } };
    (@inc_ord $cb:tt, 26) => { parse_util!{ @call $cb, 27 } };
    (@inc_ord $cb:tt, 27) => { parse_util!{ @call $cb, 28 } };
    (@inc_ord $cb:tt, 28) => { parse_util!{ @call $cb, 29 } };
    (@inc_ord $cb:tt, 29) => { parse_util!{ @call $cb, 30 } };
    (@inc_ord $cb:tt, 30) => { parse_util!{ @call $cb, 31 } };
    (@inc_ord $cb:tt, 31) => { parse_util!{ @call $cb, 32 } };
}
