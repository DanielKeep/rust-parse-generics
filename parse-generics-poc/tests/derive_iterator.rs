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

use std::iter::once;
use std::marker::PhantomData;

macro_rules! Iterator {
    (
        () $(pub)* struct $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            { constr, params, ltimes, tnames },
            then Iterator!(@with_generics () struct $name,),
            $($tail)*
        }
    };

    (
        @with_generics
        () struct $name:ident,
        $generics:tt,
        ($($body:tt)*)
        $($tail:tt)*
    ) => {
        parse_where! {
            { preds }, then Iterator!(@with_where () struct $name, $generics,),
            $($tail)* ($($body)*)
        }
    };

    (
        @with_where
        () struct $name:ident,
        $generics:tt,
        $preds:tt,
        ; ($(pub)* $fty:ty)
    ) => {
        Iterator! {
            @with_where
            () struct $name,
            $generics,
            $preds,
            ; ($fty,)
        }
    };

    (
        @with_where
        () struct $name:ident,
        {
            constr: [$($constr:tt)*],
            params: [$($params:tt)*],
            $($_generics_tail:tt)*
        },
        {
            preds: [$($preds:tt)*],
        },
        ; ($(pub)* $fty:ty, $(PhantomData<$_phtys:ty>),* $(,)*)
    ) => {
        Iterator! {
            @as_items
            impl<$($constr)*> Iterator for $name<$($params)*>
            where $fty: Iterator, $($preds)* {
                type Item = <$fty as Iterator>::Item;
                fn next(&mut self) -> Option<Self::Item> {
                    Iterator::next(&mut self.0)
                }
                fn size_hint(&self) -> (usize, Option<usize>) {
                    Iterator::size_hint(&self.0)
                }
            }
        }
    };

    (@as_items $($its:item)*) => { $($its)* };
}

custom_derive! {
    #[derive(Iterator)]
    struct IterA(Box<Iterator<Item=&'static str>>);
}

#[test]
fn test_iter_a() {
    let mut it = IterA(Box::new(once("upon a time")));
    assert_eq!(it.next(), Some("upon a time"));
    assert_eq!(it.next(), None);
}

custom_derive! {
    #[derive(Iterator)]
    struct IterB<'a>(Box<Iterator<Item=&'a str> + 'a>);
}

#[test]
fn test_iter_b() {
    let mut it = IterB::<'static>(Box::new(once("upon a time")));
    assert_eq!(it.next(), Some("upon a time"));
    assert_eq!(it.next(), None);
}

custom_derive! {
    #[derive(Iterator)]
    struct IterC<'a, T: ?Sized + 'a>(Box<Iterator<Item=&'a T> + 'a>);
}

#[test]
fn test_iter_c() {
    let mut it = IterC::<'static, str>(Box::new(once("upon a time")));
    assert_eq!(it.next(), Some("upon a time"));
    assert_eq!(it.next(), None);
}

custom_derive! {
    #[derive(Iterator)]
    struct IterD<'a, T: ?Sized + 'a>(Box<Iterator<Item=&'a T> + 'a>)
    where T: Copy;
}

#[test]
fn test_iter_d() {
    const S: &'static &'static str = &"times in our lives";
    let mut it = IterD(Box::new(once(S)));
    assert_eq!(it.next(), Some(&"times in our lives"));
    assert_eq!(it.next(), None);
}

custom_derive! {
    #[derive(Iterator)]
    struct IterE<T>(Box<Iterator<Item=T>>, PhantomData<T>)
    where T: Copy + Clone;
}

#[test]
fn test_iter_e() {
    const S: &'static &'static str = &"times in our lives";
    let mut it = IterE(Box::new(once(S)), PhantomData);
    assert_eq!(it.next(), Some(&"times in our lives"));
    assert_eq!(it.next(), None);
}

custom_derive! {
    #[derive(Iterator)]
    struct IterF<I: Iterator>(I);
}

#[test]
fn test_iter_f() {
    let it = IterF(once("you pop"));
    for e in it {
        assert!(e != "stop");
    }
}

custom_derive! {
    #[derive(Iterator)]
    struct IterG<I>(I) where I: Iterator;
}

#[test]
fn test_iter_g() {
    let it = IterG(once("you pop"));
    for e in it {
        assert!(e != "stop");
    }
}
