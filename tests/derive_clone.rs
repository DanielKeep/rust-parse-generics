#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate parse_macros;

macro_rules! Clone_mac {
    (
        () $($tail:tt)*
    ) => {
        parse_item! {
            then Clone_mac! { @item },
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
                preds: [$($preds:tt)*],
            },
            variants: [$($vars:tt,)*],
            $($_enum_tail:tt)*
        }
    ) => {
        Clone_mac! {
            @inject_where
            (impl<$($constr)*> Clone for $name<$($params)*>),
            where ($($tnames: Clone,)* $($preds)*)
            ({
                fn clone(&self) -> Self {
                    match *self {
                        $(
                            Clone_mac!(@var_match_pat $name, $vars)
                            => Clone_mac!(@var_match_body $name, $vars),
                        )*
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
                preds: [$($preds:tt)*],
            },
            kind: unitary,
            fields: [],
            $($_struct_tail:tt)*
        }
    ) => {
        Clone_mac! {
            @inject_where
            (impl<$($constr)*> Clone for $name<$($params)*>),
            where ($($tnames: Clone,)* $($preds)*)
            ({
                fn clone(&self) -> Self {
                    $name
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
                preds: [$($preds:tt)*],
            },
            kind: tuple,
            fields: [$(
                {
                    ord: ($_ford:tt, $ford_ident:ident),
                    attrs: $_fattrs:tt,
                    vis: $_fvis:tt,
                    ty: $_fty:ty,
                },
            )*],
            $($_struct_tail:tt)*
        }
    ) => {
        Clone_mac! {
            @inject_where
            (impl<$($constr)*> Clone for $name<$($params)*>),
            where ($($tnames: Clone,)* $($preds)*)
            ({
                fn clone(&self) -> Self {
                    match *self {
                        $name($(ref $ford_ident),*) => $name($(Clone::clone($ford_ident)),*)
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
                preds: [$($preds:tt)*],
            },
            kind: record,
            fields: [$(
                {
                    ord: $_ford:tt,
                    attrs: $_fattrs:tt,
                    vis: $_fvis:tt,
                    name: $fname:ident,
                    ty: $_fty:ty,
                },
            )*],
            $($_struct_tail:tt)*
        }
    ) => {
        Clone_mac! {
            @inject_where
            (impl<$($constr)*> Clone for $name<$($params)*>),
            where ($($tnames: Clone,)* $($preds)*)
            ({
                fn clone(&self) -> Self {
                    match *self {
                        $name { $(ref $fname),* } => $name { $($fname: Clone::clone($fname)),* }
                    }
                }
            })
        }
    };

    (
        @var_match_pat
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: (),
            num_fields: 0,
        }
    ) => {
        $name::$vname
    };

    (
        @var_match_body
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: (),
            num_fields: 0,
        }
    ) => {
        $name::$vname
    };

    (
        @var_match_pat
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
        $name::$vname($(ref $ford_ident,)+)
    };

    (
        @var_match_body
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
        $name::$vname($(Clone::clone($ford_ident),)+)
    };

    (
        @var_match_pat
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: record,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: $_ford:tt,
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
        $name::$vname { $(ref $fname,)+ }
    };

    (
        @var_match_body
        $name:ident,
        {
            ord: $_ord:tt,
            attrs: $_attrs:tt,
            kind: record,
            name: $vname:ident,
            fields: [
                $(
                    {
                        ord: $_ford:tt,
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
        $name::$vname { $($fname: Clone::clone($fname),)+ }
    };

    (
        @inject_where
        ($($before:tt)*),
        where ($(,)*)
        ($($after:tt)*)
    ) => {
        Clone_mac! {
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
        Clone_mac! {
            @as_item
            $($before)* where $($preds)* $($after)*
        }
    };

    (@as_item $i:item) => { $i };
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumA {}
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumB { A }
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumC { A, B, C }
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumD { A, B(i32), C(u8, u8, u8) }
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumE { A { r: u8, g: u8, b: u8, } }
}

custom_derive! {
    #[derive(Clone_mac)]
    enum EnumF<T> { A { r: T, g: T, b: T, } }
}

custom_derive! {
    #[derive(Clone_mac)]
    struct StructA;
}

custom_derive! {
    #[derive(Clone_mac)]
    struct StructB(i32);
}

custom_derive! {
    #[derive(Clone_mac)]
    struct StructC(i32, u8, String);
}

custom_derive! {
    #[derive(Clone_mac)]
    struct StructD {
        /// The red stuff.
        r: u8,
        pub g: u8,
        b: u8,
    }
}

custom_derive! {
    #[derive(Copy, Clone_mac)]
    struct StructE<T> {
        /// The red stuff.
        r: T,
        pub g: T,
        b: T,
    }
}

#[test]
fn test_derive_clone() {
    if false { let _: EnumA = panic!(); }
    let _ = EnumB::A.clone();
    let _ = EnumC::A.clone();
    let _ = EnumC::B.clone();
    let _ = EnumC::C.clone();
    let _ = EnumD::A.clone();
    let _ = EnumD::B(42).clone();
    let _ = EnumD::C(1, 2, 3).clone();
    let _ = (EnumE::A { r: 1, g: 2, b: 3 }).clone();
    let _ = (EnumF::A { r: 1, g: 2, b: 3 }).clone();
    let _ = StructA.clone();
    let _ = StructB(42).clone();
    let _ = StructC(42, 2, String::from("hi!")).clone();
    let _ = (StructD { r: 1, g: 2, b: 3 }).clone();
}
