/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg(channel="nightly")]
#![feature(intrinsics)]
#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate parse_macros;

use std::cmp::Ordering;

extern "rust-intrinsic" {
    pub fn discriminant_value<T>(v: &T) -> u64;
}

macro_rules! PartialOrd_mac {
    (
        () $($tail:tt)*
    ) => {
        parse_item! {
            then PartialOrd_mac! { @item },
            $($tail)*
        }
    };

    (
        @item
        enum {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            variants: [],
            $($_enum_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
                    Some(Ordering::Equal)
                }
            })
        }
    };

    (
        @item
        enum {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            variants: [$var:tt,],
            $($_enum_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    match (self, other) {
                        PartialOrd_mac!(@var_match_pat other, $name, $var) => PartialOrd_mac!(@var_match_body other, $name, $var),
                    }
                }
            })
        }
    };

    (
        @item
        enum {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            variants: [$($vars:tt,)*],
            $($_enum_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    let sd = unsafe { discriminant_value(self) };
                    let od = unsafe { discriminant_value(other) };

                    if sd != od {
                        return sd.partial_cmp(&od);
                    }

                    match (self, other) {
                        $(
                            PartialOrd_mac!(@var_match_pat other, $name, $vars) => PartialOrd_mac!(@var_match_body other, $name, $vars),
                        )*
                        _ => unreachable!()
                    }
                }
            })
        }
    };

    (
        @item
        struct {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            kind: unitary,
            fields: [],
            $($_struct_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
                    Some(Ordering::Equal)
                }
            })
        }
    };

    (
        @item
        struct {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            kind: tuple,
            fields: [$(
                {
                    ord: ($ford:tt, $_ford_ident:ident),
                    attrs: $_fattrs:tt,
                    vis: $_fvis:tt,
                    ty: $_fty:ty,
                },
            )*],
            $($_struct_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    $(
                        PartialOrd_mac!(@as_expr
                            match (self.$ford).partial_cmp(&other.$ford) {
                                Some(Ordering::Equal) => (),
                                other => return other
                            }
                        );
                    )*
                    Some(Ordering::Equal)
                }
            })
        }
    };

    (
        @item
        struct {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [$($constr:tt)*],
                params: [$($params:tt)*],
                ltimes: $_ltimes:tt,
                tnames: [$($tnames:ident,)*],
            },
            where: {
                clause: $_clause:tt,
                preds: [$($preds:tt)*],
            },
            kind: record,
            fields: [$(
                {
                    ord: $_ford:tt,
                    attrs: $_fattrs:tt,
                    vis: $_fvis:tt,
                    ty: $_fty:ty,
                    name: $fname:ident,
                },
            )*],
            $($_struct_tail:tt)*
        }
    ) => {
        PartialOrd_mac! {
            @inject_where
            (impl<$($constr)*> PartialOrd for $name<$($params)*>),
            where ($($tnames: PartialOrd,)* $($preds)*)
            ({
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    $(
                        match self.$fname.partial_cmp(&other.$fname) {
                            Some(Ordering::Equal) => (),
                            other => return other
                        }
                    )*
                    Some(Ordering::Equal)
                }
            })
        }
    };

    (
        @var_match_pat
        $_other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: [],
            num_fields: 0,
        }
    ) => {
        (&$name::$vname, &$name::$vname)
    };

    (
        @var_match_body
        $_other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: [],
            num_fields: 0,
        }
    ) => {
        Some(Ordering::Equal)
    };

    (
        @var_match_pat
        $other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: tuple,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: ($_ford:tt, $ford_ident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                    },
                )+
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        (&$name::$vname($(ref $ford_ident,)+), $other)
    };

    (
        @var_match_body
        $other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: tuple,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: ($ford:tt, $ford_ident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                    },
                )+
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        {
            let lhs = ($($ford_ident,)+);
            match $other {
                &$name::$vname($(ref $ford_ident,)+) => {
                    let rhs = ($($ford_ident,)+);
                    $(
                        match PartialOrd_mac!(@as_expr (lhs.$ford).partial_cmp(&rhs.$ford)) {
                            Some(Ordering::Equal) => (),
                            other => return other
                        }
                    )+
                    Some(Ordering::Equal)
                },
                _ => unreachable!()
            }
        }
    };

    (
        @var_match_pat
        $_other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: record,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: ($_ford:tt, $ford_ident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                        name: $fname:ident,
                    },
                )+
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        (&$name::$vname { $(ref $fname,)+ },
            &$name::$vname { $($fname: ref $ford_ident,)+ })
    };

    (
        @var_match_body
        $_other:ident,
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: record,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: ($_ford:tt, $ford_ident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                        name: $fname:ident,
                    },
                )+
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        {
            $(
                match $fname.partial_cmp(&$ford_ident) {
                    Some(Ordering::Equal) => (),
                    other => return other
                }
            )+
            Some(Ordering::Equal)
        }
    };

    (
        @inject_where
        ($($before:tt)*),
        where ($(,)*)
        ($($after:tt)*)
    ) => {
        PartialOrd_mac! {
            @as_item
            $($before)* $($after)*
        }
    };

    (
        @inject_where
        ($($before:tt)*),
        where ($($preds:tt)+)
        ($($after:tt)*)
    ) => {
        PartialOrd_mac! {
            @as_item
            $($before)* where $($preds)* $($after)*
        }
    };

    (@as_expr $e:expr) => { $e };
    (@as_item $i:item) => { $i };
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumA {}
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumB { A }
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumC { A, B, C }
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumD { A, B(i32), C(u8, u8, u8) }
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumE { A { r: u8, g: u8, b: u8, } }
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    enum EnumF<T> { A { r: T, g: T, b: T, } }
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    struct StructA;
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    struct StructB(i32);
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    struct StructC(i32, u8, String);
}

custom_derive! {
    #[derive(PartialEq, PartialOrd_mac)]
    struct StructD {
        /// The red stuff.
        r: u8,
        pub g: u8,
        b: u8,
    }
}

custom_derive! {
    #[derive(Clone, PartialEq, PartialOrd_mac)]
    struct StructE<T> {
        /// The red stuff.
        r: T,
        pub g: T,
        b: T,
    }
}

#[test]
fn test_partial_ord() {
    if false { let _x: EnumA = panic!(); _x.partial_cmp(&_x); }
    { let x = EnumB::A; x.partial_cmp(&x); }
    { let x = EnumC::A; x.partial_cmp(&x); }
    { let x = EnumC::B; x.partial_cmp(&x); }
    { let x = EnumC::C; x.partial_cmp(&x); }
    { let x = EnumD::A; x.partial_cmp(&x); }
    { let x = EnumD::B(42); x.partial_cmp(&x); }
    { let x = EnumD::C(1, 2, 3); x.partial_cmp(&x); }
    { let x = EnumE::A { r: 1, g: 2, b: 3 }; x.partial_cmp(&x); }
    { let x = EnumF::A { r: 1, g: 2, b: 3 }; x.partial_cmp(&x); }
    { let x = StructA; x.partial_cmp(&x); }
    { let x = StructB(42); x.partial_cmp(&x); }
    { let x = StructC(42, 2, String::from("hi!")); x.partial_cmp(&x); }
    { let x = StructD { r: 1, g: 2, b: 3 }; x.partial_cmp(&x); }
    { let x = StructE { r: 1, g: 2, b: 3 }; x.partial_cmp(&x); }
}
