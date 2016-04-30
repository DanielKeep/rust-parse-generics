#[doc(hidden)]
#[macro_export]
macro_rules! parse_macros_util {
    (
        @call
        ($cb:ident!($($cb_arg:tt)*)),
        $($output:tt)*
    ) => {
        $cb!(
            $($cb_arg)*
            $($output)*
        )
    };

    (
        @call
        ($cb:ident!{$($cb_arg:tt)*}),
        $($output:tt)*
    ) => {
        $cb! {
            $($cb_arg)*
            $($output)*
        }
    };

    (@inc_ord_ident $cb:tt,  0) => { parse_macros_util!{ @call $cb,  1, _ord_01 } };
    (@inc_ord_ident $cb:tt,  1) => { parse_macros_util!{ @call $cb,  2, _ord_02 } };
    (@inc_ord_ident $cb:tt,  2) => { parse_macros_util!{ @call $cb,  3, _ord_03 } };
    (@inc_ord_ident $cb:tt,  3) => { parse_macros_util!{ @call $cb,  4, _ord_04 } };
    (@inc_ord_ident $cb:tt,  4) => { parse_macros_util!{ @call $cb,  5, _ord_05 } };
    (@inc_ord_ident $cb:tt,  5) => { parse_macros_util!{ @call $cb,  6, _ord_06 } };
    (@inc_ord_ident $cb:tt,  6) => { parse_macros_util!{ @call $cb,  7, _ord_07 } };
    (@inc_ord_ident $cb:tt,  7) => { parse_macros_util!{ @call $cb,  8, _ord_08 } };
    (@inc_ord_ident $cb:tt,  8) => { parse_macros_util!{ @call $cb,  9, _ord_09 } };
    (@inc_ord_ident $cb:tt,  9) => { parse_macros_util!{ @call $cb, 10, _ord_10 } };
    (@inc_ord_ident $cb:tt, 10) => { parse_macros_util!{ @call $cb, 11, _ord_11 } };
    (@inc_ord_ident $cb:tt, 11) => { parse_macros_util!{ @call $cb, 12, _ord_12 } };
    (@inc_ord_ident $cb:tt, 12) => { parse_macros_util!{ @call $cb, 13, _ord_13 } };
    (@inc_ord_ident $cb:tt, 13) => { parse_macros_util!{ @call $cb, 14, _ord_14 } };
    (@inc_ord_ident $cb:tt, 14) => { parse_macros_util!{ @call $cb, 15, _ord_15 } };
    (@inc_ord_ident $cb:tt, 15) => { parse_macros_util!{ @call $cb, 16, _ord_16 } };
    (@inc_ord_ident $cb:tt, 16) => { parse_macros_util!{ @call $cb, 17, _ord_17 } };
    (@inc_ord_ident $cb:tt, 17) => { parse_macros_util!{ @call $cb, 18, _ord_18 } };
    (@inc_ord_ident $cb:tt, 18) => { parse_macros_util!{ @call $cb, 19, _ord_19 } };
    (@inc_ord_ident $cb:tt, 19) => { parse_macros_util!{ @call $cb, 20, _ord_20 } };
    (@inc_ord_ident $cb:tt, 20) => { parse_macros_util!{ @call $cb, 21, _ord_21 } };
    (@inc_ord_ident $cb:tt, 21) => { parse_macros_util!{ @call $cb, 22, _ord_22 } };
    (@inc_ord_ident $cb:tt, 22) => { parse_macros_util!{ @call $cb, 23, _ord_23 } };
    (@inc_ord_ident $cb:tt, 23) => { parse_macros_util!{ @call $cb, 24, _ord_24 } };
    (@inc_ord_ident $cb:tt, 24) => { parse_macros_util!{ @call $cb, 25, _ord_25 } };
    (@inc_ord_ident $cb:tt, 25) => { parse_macros_util!{ @call $cb, 26, _ord_26 } };
    (@inc_ord_ident $cb:tt, 26) => { parse_macros_util!{ @call $cb, 27, _ord_27 } };
    (@inc_ord_ident $cb:tt, 27) => { parse_macros_util!{ @call $cb, 28, _ord_28 } };
    (@inc_ord_ident $cb:tt, 28) => { parse_macros_util!{ @call $cb, 29, _ord_29 } };
    (@inc_ord_ident $cb:tt, 29) => { parse_macros_util!{ @call $cb, 30, _ord_30 } };
    (@inc_ord_ident $cb:tt, 30) => { parse_macros_util!{ @call $cb, 31, _ord_31 } };
    (@inc_ord_ident $cb:tt, 31) => { parse_macros_util!{ @call $cb, 32, _ord_32 } };
}
