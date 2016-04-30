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
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_as_str;
#[macro_use] extern crate parse_generics_shim;
#[macro_use] extern crate parse_macros;

custom_derive! {
    #[allow(dead_code)]
    #[derive(enum_as_str)]
    enum Dagashi {
        Umaibou,
        PotatoFries,
        CoffeeMilk,
        YoungDonuts,
        KinakoBou,
        NamaikiBeer,
        FueRamune,
        Menko,
    }
}

fn main() {
    println!("{}", Dagashi::FueRamune.as_str());
}
