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
        "{ \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } ,"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(W),),
        "W { \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } ,"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <>),
        "{ \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } ,"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), X),
        "{ \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <> X),
        "{ \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), 'a X),
        "{ \
            constr : [  ] , \
            ltimes : [  ] , \
            params : [  ] \
        } , 'a X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a> X),
        "{ \
            constr : [ 'a , ] , \
            ltimes : [ 'a , ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b> X),
        "{ \
            constr : [ 'a , 'b , ] , \
            ltimes : [ 'a , 'b , ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T> X),
        "{ \
            constr : [ T , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T, U> X),
        "{ \
            constr : [ T , U , ] , \
            ltimes : [  ] , \
            params : [ T , U , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b: 'a> X),
        "{ \
            constr : [ 'a , 'b : 'a , ] , \
            ltimes : [ 'a , 'b , ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <'a, 'b: 'a + 'c> X),
        "{ \
            constr : [ 'a , 'b : 'a + 'c , ] , \
            ltimes : [ 'a , 'b , ] , \
            params : [  ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: Copy> X),
        "{ \
            constr : [ T : Copy , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: std::marker::Copy> X),
        "{ \
            constr : [ T : std :: marker :: Copy , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: ::std::marker::Copy> X),
        "{ \
            constr : [ T : :: std :: marker :: Copy , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a> X),
        "{ \
            constr : [ T : 'a , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a + Copy + Clone> X),
        "{ \
            constr : [ T : 'a + Copy + Clone , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(), <T: 'a + 'b + Copy + Clone> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn()> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( ) , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn() -> &'c i32> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( ) -> &'c i32 , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32)> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( &'c i32 , ) , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32) -> &'d ()> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( &'c i32 , ) -> &'d () , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: for<> Copy> X),
        "{ \
            constr : [ T : Copy , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );

    assert_eq_str!(
        parse_generics!(then stringify!(),
            <T: ::std::convert::Into<String>> X),
        "{ \
            constr : [ T : :: std :: convert :: Into < String , > , ] , \
            ltimes : [  ] , \
            params : [ T , ] \
        } , X"
    );
}

#[test]
fn test_simple_where() {
    assert_eq_str!(
        parse_where!(then stringify!(),),
        "{ preds : [  ] } ,"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), X),
        "{ preds : [  ] } , X"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), {} X),
        "{ preds : [  ] } , {  } X"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), where T: Copy {X}),
        "{ preds : [ T : Copy , ] } , { X }"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), where T: Copy, {X}),
        "{ preds : [ T : Copy , ] } , { X }"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), where T: Copy; {X}),
        "{ preds : [ T : Copy , ] } , ; { X }"
    );

    assert_eq_str!(
        parse_where!(then stringify!(), where for<'a> &'a str: Into<T> {X}),
        "{ preds : [ for < 'a > &'a str : Into < T , > , ] } , { X }"
    );

    assert_eq_str!(
        parse_where!(then stringify!(),
            where for<'a: 'b, 'b> &'a str: Into<T+'b> {X}),
        "{ \
            preds : [ \
                for < 'a : 'b , 'b > &'a str : Into < T+ 'b , > , \
            ] \
        } , { X }"
    );

    assert_eq_str!(
        parse_where!(then stringify!(),
            where &'a str: Into<T+'b>, 'a: 'b, 'b: 'c + 'd {X}),
        "{ \
            preds : [ \
                &'a str : Into < T+ 'b , > , \
                'a : 'b , \
                'b : 'c + 'd , \
            ] \
        } , { X }"
    );
}
