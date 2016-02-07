/*wrap_fn*/#![feature(plugin)]
#![plugin(parse_generics_poc)]

use std::fmt::Display;

macro_rules! wrap_fn {
    (
        as $wrap_name:ident,
        fn $fn_name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            then wrap_fn!(@expand $wrap_name, $fn_name,),
            $($tail)*
        }
    };

    (@as_item $i:item) => { $i };

    (
        @expand
        $wrap_name:ident,
        $fn_name:ident,
        {
            constr: [$($constr:tt)*],
            params: [$($params:tt)*],
            $($_generics_tail:tt)*
        },
        (
            $($arg_pats:ident: $arg_tys:ty),* $(,)*
        )
        $(-> $ret_ty:ty)* ;
        pre { $($pre:tt)* }
        post($res:ident) { $($post:tt)* }
    ) => {
        wrap_fn! {
            @as_item
            fn $wrap_name<$($constr)*>($($arg_pats: $arg_tys),*) $(-> $ret_ty)* {
                $($pre)*
                let $res = $fn_name::<$($params)*>($($arg_pats),*);
                $($post)*
            }
        }
    };
}

wrap_fn! {
    as wrap_to_string,
    fn to_string<T: Display>(v: T) -> String;
    pre {
        let prefix = String::from("wrap:");
    }
    post(result) {
        format!("{}{}", prefix, result)
    }
}

fn to_string<T: Display>(v: T) -> String {
    format!("{}", v)
}

#[test]
fn test_wrap_fn() {
    assert_eq!(&*wrap_to_string(42i32), "wrap:42");
}

trait Dummy<'a, T: ?Sized>: Sized {
    fn id(self) -> Self { self }
}

impl<'a> Dummy<'a, str> for &'a str {}

fn id<'a, T: ?Sized, U: ::Dummy<'a, T>>(v: U) -> U { v.id() }

wrap_fn! {
    as wrap_id,
    fn id<'a, T: ?Sized, U: ::Dummy<'a, T>>(v: U) -> U;
    pre {}
    post(result) { result }
}

#[test]
fn test_wrap_fn_id() {
    assert_eq!(wrap_id("hi!"), "hi!");
}
