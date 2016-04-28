#[doc(hidden)]
#[macro_export]
macro_rules! parse_generics_shim_util {
    (
        @callback
        ($cb_name:ident ! ($($cb_arg:tt)*)),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! [$($cb_arg:tt)*]),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! {$($cb_arg:tt)*}),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };
}

mod parse_constr;
mod parse_generics_shim;
mod parse_where_shim;
