#[doc(hidden)]
#[macro_export]
macro_rules! parse_item {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub enum $($tail:tt)*
    ) => {
        parse_enum! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* pub enum $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* enum $($tail:tt)*
    ) => {
        parse_enum! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* enum $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub struct $($tail:tt)*
    ) => {
        parse_struct! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* pub struct $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* struct $($tail:tt)*
    ) => {
        parse_struct! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* struct $($tail)*
        }
    };
}
