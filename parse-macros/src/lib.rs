#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

#[macro_use] mod parse_enum;
#[macro_use] mod parse_item;
#[macro_use] mod parse_macros_util;
#[macro_use] mod parse_struct;
