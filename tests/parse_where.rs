#![cfg_attr(feature="use-parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="use-parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate parse_generics_shim;

macro_rules! as_item { ($i:item) => { $i } }

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

macro_rules! pwts {
    ($fields:tt, $($body:tt)*) => {
        parse_where_shim! {
            $fields,
            then stringify!(),
            $($body)*
        }
    };
}

#[test]
fn test_no_where() {
    aeqiws!(
        pwts!({..}, X),
        r#"
            { preds : [ ] , .. } ,
            X
        "#
    );

    aeqiws!(
        pwts!({ preds }, X),
        r#"
            { preds : [ ] , } ,
            X
        "#
    );
}

#[test]
fn test_where() {
    aeqiws!(
        pwts!({..}, where 'a: 'b; X),
        r#"
            { preds : [ 'a : 'b , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where T: 'a + U; X),
        r#"
            { preds : [ T : 'a + U , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where 'a: 'b, T: 'a + U; X),
        r#"
            { preds : [ 'a : 'b , T : 'a + U , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where 'a: 'b, T: 'a + U, {} X),
        r#"
            { preds : [ 'a : 'b , T : 'a + U , ] , .. } ,
            { } X
        "#
    );

    aeqiws!(
        pwts!({..}, where for<> T: 'a; X),
        r#"
            { preds : [ T : 'a , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where for<'a> T: 'a; X),
        r#"
            { preds : [ for < 'a , > T : 'a , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where for<'a: 'b> T: 'a; X),
        r#"
            { preds : [ for < 'a : 'b , > T : 'a , ] , .. } ,
            ; X
        "#
    );

    aeqiws!(
        pwts!({..}, where 'a: 'b, for<'a: 'b> T: 'a, 'c: 'a + 'b; X),
        r#"
            { preds : [ 'a : 'b , for < 'a : 'b , > T : 'a , 'c : 'a + 'b , ] , .. } ,
            ; X
        "#
    );
}
