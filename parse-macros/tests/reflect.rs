#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate parse_macros;

macro_rules! Reflect {
    (() $($tail:tt)*) => {
        parse_item! {
            then Reflect! { @item },
            $($tail)*
        }
    };

    (@as_item $i:item) => { $i };

    (
        @item
        struct {
            attrs: $_attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: {
                constr: [ $($constr:tt)* ],
                params: [ $($params:tt)* ],
                ltimes: [ $($ltimes:tt,)* ],
                tnames: [ $($tnames:ident,)* ],
            },
            where: {
                preds: [ $($preds:tt)* ],
            },
            kind: $_kind:tt,
            fields: $fields:tt,
            $($_tail:tt)*
        }
    ) => {
        Reflect! {
            @as_item
            impl<$($constr)*> Reflect for $name<$($params)*>
            where $($tnames: Reflect,)* $($preds)* {
                fn reflect() -> Type {
                    let fields = Reflect!(@record_fields $fields);
                    let item = Item {
                        visibility: vis_to_visibility!($vis),
                        module: module_path!(),
                        name: stringify!($name),
                        kind: ItemKind::Struct(Struct {
                            fields: fields,
                        }),
                    };
                    Type::Item(item)
                }
            }
        }
    };

    (
        @record_fields
        [
            $(
                {
                    ord: $_ord:tt,
                    attrs: $_attrs:tt,
                    vis: $vis:tt,
                    name: $name:ident,
                    ty: $ty:ty,
                    $(_field_tail:tt)*
                },
            )*
        ]
    ) => {
        vec![
            $(
                RecordField {
                    visibility: vis_to_visibility!($vis),
                    name: stringify!($name),
                    ty: Box::new(<$ty as Reflect>::reflect()),
                },
            )*
        ].into_boxed_slice()
    };
}

macro_rules! vis_to_visibility {
    (()) => { Visibility::Private };
    ((pub)) => { Visibility::Public };
}

pub trait Reflect {
    fn reflect() -> Type;
}

impl Reflect for u8 {
    fn reflect() -> Type {
        Type::Lang(Lang::U8)
    }
}

impl<T> Reflect for Option<T>
where T: Reflect {
    fn reflect() -> Type {
        Type::Item(Item {
            visibility: Visibility::Public,
            module: "core::option",
            name: "Option",
            kind: ItemKind::Enum(Enum {
                variants: vec![
                    EnumVariant {
                        name: "None",
                        kind: EnumVariantKind::Unitary,
                    },
                    EnumVariant {
                        name: "Some",
                        kind: EnumVariantKind::Tuple(vec![
                            TupleField {
                                ty: Box::new(<T as Reflect>::reflect()),
                            },
                        ].into_boxed_slice()),
                    },
                ].into_boxed_slice(),
            }),
        })
    }
}

#[derive(Debug)]
pub enum Type {
    Lang(Lang),
    Item(Item),
}

#[derive(Debug)]
pub enum Lang {
    U8,
}

#[derive(Debug)]
pub struct Item {
    pub visibility: Visibility,
    pub module: &'static str,
    pub name: &'static str,
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    Enum(Enum),
    Struct(Struct),
}

#[derive(Debug)]
pub struct Enum {
    pub variants: Box<[EnumVariant]>,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: &'static str,
    pub kind: EnumVariantKind,
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Unitary,
    Tuple(Box<[TupleField]>),
    Record(Box<[RecordField]>),
}

#[derive(Debug)]
pub struct Struct {
    pub fields: Box<[RecordField]>,
}

#[derive(Debug)]
pub enum Visibility { Private, Public }

#[derive(Debug)]
pub struct TupleField {
    pub ty: Box<Type>,
}

#[derive(Debug)]
pub struct RecordField {
    pub visibility: Visibility,
    pub name: &'static str,
    pub ty: Box<Type>,
}

custom_derive! {
    #[derive(Debug)]
    #[derive(Reflect)]
    struct Rgba<T> where T: Copy {
        /// The red stuff
        r: T,
        #[doc="delicious green flavour"]
        g: T,
        pub b: T,
        /// Maybe alpha, maybe not.
        pub a: Option<T>,
    }
}

macro_rules! aeqiws {
    ($lhs:expr, $rhs:expr) => {
        {
            let lhs = $lhs.replace(char::is_whitespace, "");
            let rhs = $rhs.replace(char::is_whitespace, "");
            if lhs != rhs {
                panic!("assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)", lhs, rhs);
            }
        }
    };
}

#[test]
fn test_reflect() {
    aeqiws!(
        format!("{:?}", Rgba::<u8>::reflect()),
        r#"
            Item(Item {
                visibility: Private,
                module: "reflect",
                name: "Rgba",
                kind: Struct(Struct {
                    fields: [
                        RecordField {
                            visibility: Private,
                            name: "r",
                            ty: Lang(U8)
                        },
                        RecordField {
                            visibility: Private,
                            name: "g",
                            ty: Lang(U8)
                        },
                        RecordField {
                            visibility: Public,
                            name: "b",
                            ty: Lang(U8)
                        },
                        RecordField {
                            visibility: Public,
                            name: "a",
                            ty: Item(Item {
                                visibility: Public,
                                module: "core::option",
                                name: "Option",
                                kind: Enum(Enum {
                                    variants: [
                                        EnumVariant {
                                            name: "None",
                                            kind: Unitary
                                        },
                                        EnumVariant {
                                            name: "Some",
                                            kind: Tuple([
                                                TupleField {
                                                    ty: Lang(U8)
                                                }
                                            ])
                                        }
                                    ]
                                })
                            })
                        }
                    ]
                })
            })
        "#
    );
}
