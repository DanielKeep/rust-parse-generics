#![feature(plugin)]
#![plugin(parse_generics_poc)]

macro_rules! assert_eq_str {
    ($left:expr, $right:expr) => {
        assert_eq!($left.replace("\n", " "), $right.replace("\n", " "))
    };
}

#[test]
fn test_simple() {
    assert_eq_str!(
        parse_generics!(then stringify!(),),
        "{ ltimes : [  ] , params : [  ] , constr : [  ] } ,"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <>),
        "{ ltimes : [  ] , params : [  ] , constr : [  ] } ,"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), X),
        "{ ltimes : [  ] , params : [  ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <> X),
        "{ ltimes : [  ] , params : [  ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), 'a X),
        "{ ltimes : [  ] , params : [  ] , constr : [  ] } , 'a X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a> X),
        "{ ltimes : [ 'a , ] , params : [  ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b> X),
        "{ ltimes : [ 'a , 'b , ] , params : [  ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T, U> X),
        "{ ltimes : [  ] , params : [ T , U , ] , constr : [  ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b: 'a> X),
        "{ ltimes : [ 'a , 'b , ] , params : [  ] , constr : [ 'b : 'a ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b: 'a + 'c> X),
        "{ ltimes : [ 'a , 'b , ] , params : [  ] , constr : [ 'b : 'a + 'c ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: Copy> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : Copy ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : 'a ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a + Copy + Clone> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : 'a + Copy + Clone ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a + 'b + Copy + Clone> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : 'a + 'b + Copy + Clone ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32)> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : 'a + 'b + Copy + Clone + for < 'c , 'd : 'e > Fn(&'c i32) ] } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: for<> Copy> X),
        "{ ltimes : [  ] , params : [ T , ] , constr : [ T : Copy ] } , X"
    );
}
