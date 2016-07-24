/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
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

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub type $($tail:tt)*
    ) => {
        parse_type_item! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* pub type $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* type $($tail:tt)*
    ) => {
        parse_type_item! {
            then $cb!$cb_arg,
            $(#[$($attrs)*])* type $($tail)*
        }
    };
}
