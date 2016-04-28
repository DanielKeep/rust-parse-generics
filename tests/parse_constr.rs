#[macro_use] extern crate parse_generics_shim;

macro_rules! aeqiws {
    ($lhs:expr, $rhs:expr) => {
        {
            let lhs = $lhs;
            let rhs = $rhs;
            let lhs_words = $lhs.split_whitespace();
            let rhs_words = $rhs.split_whitespace();
            for (i, (l, r)) in lhs_words.zip(rhs_words).enumerate() {
                if l != r {
                    panic!("assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`, at word {}, `{:?}` != `{:?}`)", lhs, rhs, i, l, r);
                }
            }
        }
    };
}

macro_rules! pgts {
    ($($body:tt)*) => {
        parse_constr! {
            (true, true),
            then stringify!(),
            $($body)*
        }
    };
}

#[test]
fn test_simple() {
    aeqiws!(pgts!('a, X), "{ 'a } , , X");
    aeqiws!(pgts!('a> X), "{ 'a } , > X");
    aeqiws!(pgts!('a {X}), "{ 'a } , { X }");
    aeqiws!(pgts!('a; X), "{ 'a } , ; X");
    aeqiws!(pgts!('a = X), "{ 'a } , = X");

    aeqiws!(pgts!(T; X), "{ T } , ; X");
    aeqiws!(pgts!('a + T; X), "{ 'a + T } , ; X");
    aeqiws!(pgts!('a + 'b; X), "{ 'a + 'b } , ; X");
    aeqiws!(pgts!('a + 'b + T; X), "{ 'a + 'b + T } , ; X");
    aeqiws!(pgts!('a + ::std::clone::Clone; X), "{ 'a + :: std:: clone:: Clone } , ; X");
    aeqiws!(pgts!('a + From<u8>; X), "{ 'a + From < u8 > } , ; X");
    aeqiws!(pgts!('a + From<Bar<u8>>; X), "{ 'a + From < Bar < u8 >> } , ; X");
}
