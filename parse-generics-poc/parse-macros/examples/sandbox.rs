#![feature(plugin)]
#![plugin(parse_generics_poc)]

#[macro_use] extern crate parse_macros;

fn main() {
    println!("{}\n", parse_struct!(
        then stringify!(),
        /// I have a comment!
        #[derive(Copy, Clone)]
        struct Unitary;
    ));

    println!("{}\n", parse_struct!(
        then stringify!(),
        struct Rgb<T>(T, T, T) where T: Copy;
    ));

    println!("{}\n", parse_struct!(
        then stringify!(),
        struct Rgba<T> where T: Copy {
            /// The red stuff
            r: T,
            #[doc="delicious green flavour"]
            g: T,
            pub b: T,
            /// Maybe alpha, maybe not.
            pub a: Option<T>,
        }
    ));

    println!("{}\n", parse_enum!(
        then stringify!(),
        enum Boolean { True, False, FileNotFound }
    ));

    println!("{}\n", parse_enum!(
        then stringify!(),
        enum Option<T> { Some(T), None }
    ));

    println!("{}\n", parse_enum!(
        then stringify!(),
        enum Optiony<T> { Some { pub value: T }, None }
    ));
}
