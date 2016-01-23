#![feature(plugin)]
#![plugin(parse_generics_poc)]
#[macro_use] extern crate custom_derive;

use std::iter::once;

macro_rules! Iterator {
    (
        () $(pub)* struct $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            then Iterator!(@with_generics () struct $name,),
            $($tail)*
        }
    };

    (
        @with_generics
        () struct $name:ident,
        {
            constr: [$($constr:tt)*],
            ltimes: [$($ltimes:tt)*],
            params: [$($params:tt)*]
        },
        ($(pub)* $fty:ty);
    ) => {
        Iterator! {
            @as_items
            impl<$($constr)*> Iterator for $name<$($ltimes)* $($params)*>
            where $fty: Iterator {
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
