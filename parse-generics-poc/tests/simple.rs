/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
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
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } ,"
    );

    assert_eq_str!(
        parse_generics!({ .. }, then stringify!(),),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
            .. \
        } ,"
    );

    assert_eq_str!(
        parse_generics!({ params, mrtnames?, tnames?, bunnies? }, then stringify!(),),
        "{ \
            params : [  ] , \
            tnames : [  ] , \
        } ,"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(W),),
        "W { \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } ,"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <>),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } ,"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), X),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <> X),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), 'a X),
        "{ \
            constr : [  ] , \
            params : [  ] , \
            ltimes : [  ] , \
            tnames : [  ] , \
        } , 'a X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <'a> X),
        "{ \
            constr : [ 'a , ] , \
            params : [ 'a , ] , \
            ltimes : [ 'a , ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <'a, 'b> X),
        "{ \
            constr : [ 'a , 'b , ] , \
            params : [ 'a , 'b , ] , \
            ltimes : [ 'a , 'b , ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T> X),
        "{ \
            constr : [ T , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T, U> X),
        "{ \
            constr : [ T , U , ] , \
            params : [ T , U , ] , \
            ltimes : [  ] , \
            tnames : [ T , U , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <'a, 'b: 'a> X),
        "{ \
            constr : [ 'a , 'b : 'a , ] , \
            params : [ 'a , 'b , ] , \
            ltimes : [ 'a , 'b , ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <'a, 'b: 'a + 'c> X),
        "{ \
            constr : [ 'a , 'b : 'a + 'c , ] , \
            params : [ 'a , 'b , ] , \
            ltimes : [ 'a , 'b , ] , \
            tnames : [  ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: Copy> X),
        "{ \
            constr : [ T : Copy , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: std::marker::Copy> X),
        "{ \
            constr : [ T : std :: marker :: Copy , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: ::std::marker::Copy> X),
        "{ \
            constr : [ T : :: std :: marker :: Copy , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: 'a> X),
        "{ \
            constr : [ T : 'a , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: 'a + Copy + Clone> X),
        "{ \
            constr : [ T : 'a + Copy + Clone , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(), <T: 'a + 'b + Copy + Clone> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn()> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( ) , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn() -> &'c i32> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( ) -> &'c i32 , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32)> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( &'c i32 ) , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32) -> &'d ()> X),
        "{ \
            constr : [ T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( &'c i32 ) -> &'d () , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <'a, 'b, 'd, T: 'a + 'b + Copy + Clone + for<'c, 'd: 'e> Fn(&'c i32) -> &'d ()> X),
        "{ \
            constr : [ 'a , 'b , 'd , T : 'a + 'b + Copy + Clone \
                + for < 'c , 'd : 'e > Fn ( &'c i32 ) -> &'d () , ] , \
            params : [ 'a , 'b , 'd , T , ] , \
            ltimes : [ 'a , 'b , 'd , ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: for<> Copy> X),
        "{ \
            constr : [ T : Copy , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params, ltimes, tnames }, then stringify!(),
            <T: ::std::convert::Into<String>> X),
        "{ \
            constr : [ T : :: std :: convert :: Into < String > , ] , \
            params : [ T , ] , \
            ltimes : [  ] , \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({}, then stringify!(), <'a: 'b, T: U> X),
        "{  } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            constr : [ 'a : 'b , T : U , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ params }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            params : [ 'a , T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ ltimes }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            ltimes : [ 'a , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ tnames }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            tnames : [ T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ constr, params }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            constr : [ 'a : 'b , T : U , ] , \
            params : [ 'a , T , ] , \
        } , X"
    );

    assert_eq_str!(
        parse_generics!({ params, constr }, then stringify!(), <'a: 'b, T: U> X),
        "{ \
            params : [ 'a , T , ] , \
            constr : [ 'a : 'b , T : U , ] , \
        } , X"
    );
}

#[test]
fn test_simple_where() {
    assert_eq_str!(
        parse_where!({ clause, preds }, then stringify!(),),
        "{ clause : [  ] , preds : [  ] , } ,"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(),),
        "{ preds : [  ] , } ,"
    );

    assert_eq_str!(
        parse_where!({ .. }, then stringify!(),),
        "{ clause : [  ] , preds : [  ] , .. } ,"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), X),
        "{ preds : [  ] , } , X"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), {} X),
        "{ preds : [  ] , } , {  } X"
    );

    assert_eq_str!(
        parse_where!({ clause }, then stringify!(), where T: Copy {X}),
        "{ clause : [ where T : Copy , ] , } , { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), where T: Copy {X}),
        "{ preds : [ T : Copy , ] , } , { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), where T: Copy, {X}),
        "{ preds : [ T : Copy , ] , } , { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), where T: Copy; {X}),
        "{ preds : [ T : Copy , ] , } , ; { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(), where for<'a> &'a str: Into<T> {X}),
        "{ preds : [ for < 'a , > &'a str : Into < T > , ] , } , { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(),
            where for<'a: 'b, 'b> &'a str: Into<T+'b> {X}),
        "{ \
            preds : [ \
                for < 'a : 'b , 'b , > &'a str : Into < T+ 'b > , \
            ] , \
        } , { X }"
    );

    assert_eq_str!(
        parse_where!({ preds }, then stringify!(),
            where &'a str: Into<T+'b>, 'a: 'b, 'b: 'c + 'd {X}),
        "{ \
            preds : [ \
                &'a str : Into < T+ 'b > , \
                'a : 'b , \
                'b : 'c + 'd , \
            ] , \
        } , { X }"
    );

    assert_eq_str!(
        parse_where!({}, then stringify!(), where T: Copy {X}),
        "{  } , { X }"
    );
}
