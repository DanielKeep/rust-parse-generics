/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate parse_macros;

use serde::ser::{Serialize, Serializer};

macro_rules! Serialize {
    (
        () $($tail:tt)*
    ) => {
        parse_item! {
            then Serialize! { @item },
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
        Serialize! {
            @inject_where
            (impl<$($constr)*> Serialize for $name<$($params)*>),
            where ($($tnames: Serialize,)* $($preds)*)
            ({
                fn serialize<Ser: Serializer>(&self, _: &mut Ser) -> Result<(), Ser::Error> {
                    unreachable!();
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
        Serialize! {
            @inject_where
            (impl<$($constr)*> Serialize for $name<$($params)*>),
            where ($($tnames: Serialize,)* $($preds)*)
            ({
                fn serialize<Ser: Serializer>(&self, ser: &mut Ser) -> Result<(), Ser::Error> {
                    match self {
                        $(
                            Serialize!(@var_pat $name, $vars) => Serialize!(@var_body $name, ser, $vars),
                        )*
                    }
                }
            })
        }
    };

    (
        @var_pat
        $name:ident,
        {
            ord: $_vord:tt,
            attrs: $_vattrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: [],
            num_fields: 0,
        }
    ) => {
        &$name::$vname
    };

    (
        @var_body
        $name:ident,
        $ser:ident,
        {
            ord: ($vord:expr, $_vord_ident:ident),
            attrs: $_vattrs:tt,
            kind: unitary,
            name: $vname:ident,
            fields: [],
            num_fields: 0,
        }
    ) => {
        $ser.visit_unit_variant(stringify!($name), $vord, stringify!($vname))
    };

    (
        @var_pat
        $name:ident,
        {
            ord: $_vord:tt,
            attrs: $_vattrs:tt,
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
                )*
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        &$name::$vname($(ref $ford_ident,)*)
    };

    (
        @var_body
        $name:ident,
        $ser:ident,
        {
            ord: ($vord:expr, $_vord_ident:ident),
            attrs: $_vattrs:tt,
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
                )*
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        $ser.visit_tuple_variant(
            stringify!($name), $vord, stringify!($vname),
            ($(&$ford_ident,)*).into_visitor())
    };

    (
        @var_pat
        $name:ident,
        {
            ord: $_vord:tt,
            attrs: $_vattrs:tt,
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
                )*
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        &$name::$vname { $(ref $fname,)* }
    };

    (
        @var_body
        $name:ident,
        $ser:ident,
        {
            ord: ($vord:expr, $_vord_ident:ident),
            attrs: $_vattrs:tt,
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
                )*
            ],
            num_fields: $_num_fields:tt,
        }
    ) => {
        {
            $ser.visit_struct_variant(
                stringify!($name), $vord, stringify!($vname),
                ($((stringify!($fname), $fname),)*).into_map_visitor())
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
        Serialize! {
            @inject_where
            (impl<$($constr)*> Serialize for $name<$($params)*>),
            where ($($tnames: Serialize,)* $($preds)*)
            ({
                fn serialize<Ser: Serializer>(&self, ser: &mut Ser) -> Result<(), Ser::Error> {
                    try!(ser.visit_unit_struct(stringify!($name)));
                    Ok(())
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
            fields: [
                $(
                    {
                        ord: ($ford:tt, $_fident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                    },
                )+
            ],
            $($_struct_tail:tt)*
        }
    ) => {
        Serialize! {
            @inject_where
            (impl<$($constr)*> Serialize for $name<$($params)*>),
            where ($($tnames: Serialize,)* $($preds)*)
            ({
                fn serialize<Ser: Serializer>(&self, ser: &mut Ser) -> Result<(), Ser::Error> {
                    ser.visit_tuple_struct(stringify!($name),
                        Serialize!(@as_expr ($(&self.$ford,)*).into_visitor()))
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
            fields: [
                $(
                    {
                        ord: ($ford:tt, $fident:ident),
                        attrs: $_fattrs:tt,
                        vis: $_fvis:tt,
                        ty: $_fty:ty,
                        name: $fname:ident,
                    },
                )+
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        Serialize! {
            @inject_where
            (impl<$($constr)*> Serialize for $name<$($params)*>),
            where ($($tnames: Serialize,)* $($preds)*)
            ({
                fn serialize<Ser: Serializer>(&self, ser: &mut Ser) -> Result<(), Ser::Error> {
                    Serialize! {
                        @inject_where
                        (struct Visitor<'vis, $($constr)*>),
                        where ($($tnames: 'vis + Serialize,)* $($preds)*)
                        ({
                            off: usize,
                            ptr: &'vis $name<$($params)*>,
                        })
                    }

                    Serialize! {
                        @inject_where
                        (impl<'vis, $($constr)*> serde::ser::MapVisitor for Visitor<'vis, $($params)*>),
                        where ($($tnames: 'vis + Serialize,)* $($preds)*)
                        ({
                            fn visit<Ser>(&mut self, ser: &mut Ser) -> Result<Option<()>, Ser::Error>
                            where Ser: Serializer {
                                match self.off {
                                    $(
                                        $ford => {
                                            self.off += 1;
                                            Ok(Some(try!(ser.visit_struct_elt(stringify!($fname), &self.ptr.$fname))))
                                        },
                                    )+
                                    _ => Ok(None)
                                }
                            }

                            fn len(&self) -> Option<usize> {
                                Some($num_fields)
                            }
                        })
                    }

                    ser.visit_struct(stringify!($name), Visitor { off: 0, ptr: self })
                }
            })
        }
    };

    (
        @inject_where
        ($($before:tt)*),
        where ($(,)*)
        ($($after:tt)*)
    ) => {
        Serialize! {
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
        Serialize! {
            @as_item
            $($before)* where $($preds)* $($after)*
        }
    };

    (@as_expr $e:expr) => { $e };
    (@as_item $i:item) => { $i };
}

trait TupleExt {
    type Visitor: serde::ser::SeqVisitor;
    fn into_visitor(self) -> Self::Visitor;
}

impl<'a, T0> TupleExt for (&'a T0,)
where
    T0: 'a + Serialize,
{
    type Visitor = TupleRefVisitor1<'a, T0>;

    fn into_visitor(self) -> Self::Visitor {
        TupleRefVisitor1 { off: 0, tup: self }
    }
}

struct TupleRefVisitor1<'a, T0: 'a> {
    off: usize,
    tup: (&'a T0,),
}

impl<'a, T0> serde::ser::SeqVisitor for TupleRefVisitor1<'a, T0>
where
    T0: 'a + Serialize,
{
    fn visit<S>(&mut self, ser: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer {
        match self.off {
            0 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_tuple_elt(self.tup.0))))
            },
            _ => {
                Ok(None)
            }
        }
    }

    fn len(&self) -> Option<usize> {
        Some(1)
    }
}

impl<'a, T0, T1, T2> TupleExt for (&'a T0, &'a T1, &'a T2,)
where
    T0: 'a + Serialize,
    T1: 'a + Serialize,
    T2: 'a + Serialize,
{
    type Visitor = TupleRefVisitor3<'a, T0, T1, T2>;

    fn into_visitor(self) -> Self::Visitor {
        TupleRefVisitor3 { off: 0, tup: self }
    }
}

struct TupleRefVisitor3<'a, T0: 'a, T1: 'a, T2: 'a> {
    off: usize,
    tup: (&'a T0, &'a T1, &'a T2,),
}

impl<'a, T0, T1, T2> serde::ser::SeqVisitor for TupleRefVisitor3<'a, T0, T1, T2>
where
    T0: 'a + Serialize,
    T1: 'a + Serialize,
    T2: 'a + Serialize,
{
    fn visit<S>(&mut self, ser: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer {
        match self.off {
            0 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_tuple_elt(self.tup.0))))
            },
            1 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_tuple_elt(self.tup.1))))
            },
            2 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_tuple_elt(self.tup.2))))
            },
            _ => {
                Ok(None)
            }
        }
    }

    fn len(&self) -> Option<usize> {
        Some(3)
    }
}

trait TupleMapExt {
    type Visitor: serde::ser::MapVisitor;
    fn into_map_visitor(self) -> Self::Visitor;
}

impl<'a, T0, T1, T2> TupleMapExt for ((&'static str, &'a T0), (&'static str, &'a T1), (&'static str, &'a T2))
where
    T0: 'a + Serialize,
    T1: 'a + Serialize,
    T2: 'a + Serialize,
{
    type Visitor = MapRefVisitor3<'a, T0, T1, T2>;
    fn into_map_visitor(self) -> Self::Visitor {
        MapRefVisitor3 { off: 0, tup: self }
    }
}

struct MapRefVisitor3<'a, T0: 'a, T1: 'a, T2: 'a> {
    off: usize,
    tup: ((&'static str, &'a T0), (&'static str, &'a T1), (&'static str, &'a T2)),
}

impl<'a, T0, T1, T2> serde::ser::MapVisitor for MapRefVisitor3<'a, T0, T1, T2>
where
    T0: 'a + Serialize,
    T1: 'a + Serialize,
    T2: 'a + Serialize,
{
    fn visit<S>(&mut self, ser: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer {
        match self.off {
            0 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_struct_variant_elt((self.tup.0).0, (self.tup.0).1))))
            },
            1 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_struct_variant_elt((self.tup.1).0, (self.tup.1).1))))
            },
            2 => {
                self.off += 1;
                Ok(Some(try!(ser.visit_struct_variant_elt((self.tup.2).0, (self.tup.2).1))))
            },
            _ => Ok(None)
        }
    }
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumA {}
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumB { A }
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumC { A, B, C }
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumD { A, B(i32), C(u8, u8, u8) }
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumE { A { r: u8, g: u8, b: u8, } }
}

custom_derive! {
    #[derive(Serialize)]
    enum EnumF<T> { A { r: T, g: T, b: T, } }
}

custom_derive! {
    #[derive(Serialize)]
    struct StructA;
}

custom_derive! {
    #[derive(Serialize)]
    struct StructB(i32);
}

custom_derive! {
    #[derive(Serialize)]
    struct StructC(i32, u8, String);
}

custom_derive! {
    #[derive(Serialize)]
    struct StructD {
        /// The red stuff.
        r: u8,
        pub g: u8,
        b: u8,
    }
}

custom_derive! {
    #[derive(Clone, Serialize)]
    struct StructE<T> {
        /// The red stuff.
        r: T,
        pub g: T,
        b: T,
    }
}

macro_rules! assert_ser {
    ($e:expr, $ex:expr) => {
        match serde_json::to_string(&$e).ok() {
            r => assert_eq!(r.as_ref().map(|s| &**s), Some($ex))
        }
    };
}

#[test]
fn test_serialize() {
    if false { assert_ser!({let _x: EnumA = panic!(); _x}, "!"); }
    assert_ser!(EnumB::A, r#"{"A":[]}"#);
    assert_ser!(EnumC::A, r#"{"A":[]}"#);
    assert_ser!(EnumC::B, r#"{"B":[]}"#);
    assert_ser!(EnumC::C, r#"{"C":[]}"#);
    assert_ser!(EnumD::A, r#"{"A":[]}"#);
    assert_ser!(EnumD::B(42), r#"{"B":[42]}"#);
    assert_ser!(EnumD::C(1, 2, 3), r#"{"C":[1,2,3]}"#);
    assert_ser!(EnumE::A { r: 1, g: 2, b: 3 }, r#"{"A":{"r":1,"g":2,"b":3}}"#);
    assert_ser!(EnumF::A { r: 1, g: 2, b: 3 }, r#"{"A":{"r":1,"g":2,"b":3}}"#);

    assert_ser!(StructA, "null");
    assert_ser!(StructB(42), "[42]");
    assert_ser!(StructC(42, 2, String::from("hi!")), "[42,2,\"hi!\"]");
    assert_ser!(StructD { r: 1, g: 2, b: 3 }, r#"{"r":1,"g":2,"b":3}"#);
    assert_ser!(StructE { r: 1, g: 2, b: 3 }, r#"{"r":1,"g":2,"b":3}"#);
}
