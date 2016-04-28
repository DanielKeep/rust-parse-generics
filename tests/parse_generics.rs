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

macro_rules! pgts {
    ($fields:tt, $($body:tt)*) => {
        parse_generics_shim! {
            $fields,
            then stringify!(),
            $($body)*
        }
    };
}

#[test]
fn test_no_generics() {
    aeqiws!(
        pgts!({..}, X),
        r#"
            {
                constr : [ ] ,
                params : [ ] ,
                ltimes : [ ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({..}, <> X),
        r#"
            {
                constr : [ ] ,
                params : [ ] ,
                ltimes : [ ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ constr, params, ltimes, tnames }, X),
        r#"
            {
                constr : [ ] ,
                params : [ ] ,
                ltimes : [ ] ,
                tnames : [ ] ,
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ constr, params, ltimes, tnames }, <> X),
        r#"
            {
                constr : [ ] ,
                params : [ ] ,
                ltimes : [ ] ,
                tnames : [ ] ,
            } ,
            X
        "#
    );
}

#[test]
fn test_simple_ty_params() {
    aeqiws!(
        pgts!({ .. }, <T> X),
        r#"
            {
                constr : [ T , ] ,
                params : [ T , ] ,
                ltimes : [ ] ,
                tnames : [ T , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T, U> X),
        r#"
            {
                constr : [ T , U , ] ,
                params : [ T , U , ] ,
                ltimes : [ ] ,
                tnames : [ T , U , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T, U,> X),
        r#"
            {
                constr : [ T , U , ] ,
                params : [ T , U , ] ,
                ltimes : [ ] ,
                tnames : [ T , U , ] ,
                ..
            } ,
            X
        "#
    );
}

#[test]
fn test_constr_ty_params() {
    aeqiws!(
        pgts!({ .. }, <T: Copy> X),
        r#"
            {
                constr : [ T : Copy , ] ,
                params : [ T , ] ,
                ltimes : [ ] ,
                tnames : [ T , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T: Copy,> X),
        r#"
            {
                constr : [ T : Copy , ] ,
                params : [ T , ] ,
                ltimes : [ ] ,
                tnames : [ T , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T: Copy, U: Clone> X),
        r#"
            {
                constr : [ T : Copy , U : Clone , ] ,
                params : [ T , U , ] ,
                ltimes : [ ] ,
                tnames : [ T , U , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T: Copy, U, V: Clone,> X),
        r#"
            {
                constr : [ T : Copy , U , V : Clone , ] ,
                params : [ T , U , V , ] ,
                ltimes : [ ] ,
                tnames : [ T , U , V , ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <T: 'a, U: 'a + Copy> X),
        r#"
            {
                constr : [ T : 'a , U : 'a + Copy , ] ,
                params : [ T , U , ] ,
                ltimes : [ ] ,
                tnames : [ T , U , ] ,
                ..
            } ,
            X
        "#
    );
}

#[test]
fn test_simple_lt_params() {
    aeqiws!(
        pgts!({ .. }, <'a> X),
        r#"
            {
                constr : [ 'a , ] ,
                params : [ 'a , ] ,
                ltimes : [ 'a , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a,> X),
        r#"
            {
                constr : [ 'a , ] ,
                params : [ 'a , ] ,
                ltimes : [ 'a , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a, 'b> X),
        r#"
            {
                constr : [ 'a , 'b , ] ,
                params : [ 'a , 'b , ] ,
                ltimes : [ 'a , 'b , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a, 'b, 'i, 'z,> X),
        r#"
            {
                constr : [ 'a , 'b , 'i , 'z , ] ,
                params : [ 'a , 'b , 'i , 'z , ] ,
                ltimes : [ 'a , 'b , 'i , 'z , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );
}

#[test]
fn test_constr_lt_params() {
    aeqiws!(
        pgts!({ .. }, <'a: 'b> X),
        r#"
            {
                constr : [ 'a : 'b , ] ,
                params : [ 'a , ] ,
                ltimes : [ 'a , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a: 'b + 'c> X),
        r#"
            {
                constr : [ 'a : 'b + 'c , ] ,
                params : [ 'a , ] ,
                ltimes : [ 'a , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a: 'b + 'c,> X),
        r#"
            {
                constr : [ 'a : 'b + 'c , ] ,
                params : [ 'a , ] ,
                ltimes : [ 'a , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );

    aeqiws!(
        pgts!({ .. }, <'a: 'b + 'c, 'b: 'c, 'c> X),
        r#"
            {
                constr : [ 'a : 'b + 'c , 'b : 'c , 'c , ] ,
                params : [ 'a , 'b , 'c , ] ,
                ltimes : [ 'a , 'b , 'c , ] ,
                tnames : [ ] ,
                ..
            } ,
            X
        "#
    );
}

#[test]
fn test_passthru() {
    macro_rules! emit {
        (
            $fn_name:ident
            {
                constr: [$($constr:tt)*],
                $($_rest:tt)*
            },
            $($_tail:tt)*
        ) => {
            as_item! {
                #[allow(dead_code)]
                fn $fn_name<$($constr)*>() { panic!("BOOM!"); }
            }
        };
    }

    parse_generics_shim! { { .. }, then emit!{a}, X }
    parse_generics_shim! { { .. }, then emit!{b}, <> X }
    parse_generics_shim! { { .. }, then emit!{c}, <T> X }
    parse_generics_shim! { { .. }, then emit!{d}, <T,> X }
    parse_generics_shim! { { .. }, then emit!{e}, <T, U> X }
    parse_generics_shim! { { .. }, then emit!{f}, <T, U,> X }
    parse_generics_shim! { { .. }, then emit!{g}, <T: Copy> X }
    parse_generics_shim! { { .. }, then emit!{g2}, <T: Copy + Clone> X }
    parse_generics_shim! { { .. }, then emit!{h}, <'a> X }
    parse_generics_shim! { { .. }, then emit!{i}, <'a,> X }
    parse_generics_shim! { { .. }, then emit!{j}, <'a, 'b> X }
    parse_generics_shim! { { .. }, then emit!{k}, <'a, 'b,> X }
    parse_generics_shim! { { .. }, then emit!{l}, <'a, 'b: 'a> X }
    parse_generics_shim! { { .. }, then emit!{l2}, <'a, 'b: 'a, 'c: 'a + 'b> X }
    parse_generics_shim! { { .. }, then emit!{m}, <'a, T: 'a + Copy> X }
    parse_generics_shim! { { .. }, then emit!{m2}, <'a, T: 'a + Copy + Clone> X }

    let _ = "the rustc parser is stoopid";
}
