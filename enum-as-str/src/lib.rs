/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_export]
macro_rules! enum_as_str {
    (() $($body:tt)*) => {
        enum_as_str! { (as_str) $($body)* }
    };

    (($fn_name:ident) $($body:tt)*) => {
        parse_enum! { then enum_as_str! { @with_enum $fn_name, }, $($body)* }
    };

    (@as_item $i:item) => { $i };

    (
        @with_enum
        $fn_name:ident,
        enum {
            attrs: $_attrs:tt,
            vis: $_vis:tt,
            name: $name:ident,
            generics: {
                constr: [ $($constr:tt)* ],
                params: [ $($params:tt)* ],
                $($_generics:tt)*
            },
            where: {
                clause: [$($clause:tt)*],
                preds: $_preds:tt,
            },
            variants: [
                $(
                    {
                        ord: $_ord:tt,
                        attrs: $_vattrs:tt,
                        kind: unitary,
                        name: $vname:ident,
                        fields: [],
                        num_fields: $_vnum_fields:tt,
                    },
                )*
            ],
            num_variants: $_num_variants:tt,
        }
    ) => {
        enum_as_str! {
            @as_item
            impl<$($constr)*> $name<$($params)*> $($clause)* {
                pub fn $fn_name(&self) -> &'static str {
                    match *self {
                        $(
                            $name::$vname => stringify!($vname),
                        )*
                    }
                }
            }
        }
    };
}
