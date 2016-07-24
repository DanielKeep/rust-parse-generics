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

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate parse_macros;

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
fn test_simple_type_item() {
    aeqiws!(
        parse_item!(
            then stringify!(),
            #[doc(hidden)] type Result<T> = ::std::result::Result<T, Error>;
        ),
        r#"
            type {
                attrs: [ #[doc(hidden)] ],
                vis: (),
                name: Result,
                generics: {
                    constr: [T,],
                    params: [T,],
                    ltimes: [],
                    tnames: [T,],
                },
                where: {
                    preds: [],
                },
                type: ::std::result::Result<T, Error>,
            }
        "#
    );
}
