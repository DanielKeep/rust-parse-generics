/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![feature(plugin)]
#![plugin(parse_generics_poc)]
#[macro_use] extern crate custom_derive;

macro_rules! CloneCopy {
    (
        () $(pub)* enum $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            { constr, params, ltimes, tnames },
            then CloneCopy! { @with_generics (enum $name), },
            $($tail)*
        }
    };

    (
        () $(pub)* struct $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            { constr, params, ltimes, tnames },
            then CloneCopy! { @with_generics (struct $name), },
            $($tail)*
        }
    };

    (
        @with_generics
        $prefix:tt, $generics:tt,
        ($($body:tt)*)
        $($tail:tt)*
    ) => {
        parse_where! {
            { preds }, then CloneCopy! { @expand $prefix, $generics, },
            $($tail)* ($($body)*)
        }
    };

    (
        @with_generics
        $prefix:tt, $generics:tt,
        $($tail:tt)*
    ) => {
        parse_where! {
            { preds }, then CloneCopy! { @expand $prefix, $generics, },
            $($tail)*
        }
    };

    (
        @expand ($_kind:tt $name:ident),
        {
            constr: [$($constr:tt)*],
            params: [$($params:tt)*],
            ltimes: $_ltimes:tt,
            tnames: [],
        },
        {
            preds: [],
        },
        $($_tail:tt)*
    ) => {
        CloneCopy! {
            @as_item
            impl<$($constr)*> Clone for $name<$($params)*> {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    };

    (
        @expand ($_kind:tt $name:ident),
        {
            constr: [$($constr:tt)*],
            params: [$($params:tt)*],
            ltimes: $_ltimes:tt,
            tnames: [$($tnames:ident,)*],
        },
        {
            preds: [$($preds:tt)*]
            $($_more_preds:tt)*
        },
        $($_tail:tt)*
    ) => {
        CloneCopy! {
            @as_item
            impl<$($constr)*> Clone for $name<$($params)*>
            where $($tnames: Copy,)* $($preds)* {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    };

    (@as_item $i:item) => { $i };
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    struct Type0(u32);
}

#[test]
fn test_type_0() {
    let v = Type0(42);
    let (a, b) = (v, v);
    assert_eq!(a, b);
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    struct Type1<T> { value: T }
}

#[test]
fn test_type_1() {
    let v = Type1 { value: 42 };
    let (a, b) = (v, v);
    assert_eq!(a, b);
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    enum Type2<T> where T: Ord { A(T) }
}

#[test]
fn test_type_2() {
    let v = Type2::A(42);
    let (a, b) = (v, v);
    assert_eq!(a, b);
}
