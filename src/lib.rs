#[macro_export]
macro_rules! parse_generics_shim {
    (@as_expr $e:expr) => { $e };
    (@as_item $i:item) => { $i };

    (
        @parse_start
        $prefix:tt,
        <> $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @emit_output
            $prefix,
            {
                constr: [],
                ltimes: [],
                tnames: [],
            },
            $($tail)*
        }
    };

    (
        @parse_start
        $prefix:tt,
        < $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [],
                ltimes: [],
                tnames: [],
            },
            $($tail)*
        }
    };

    (
        @parse_start
        $prefix:tt,
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @emit_output
            $prefix,
            {
                constr: [],
                ltimes: [],
                tnames: [],
            },
            $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        $fields:tt,
        > $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @emit_output
            $prefix,
            $fields,
            $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        $fields:tt,
        $(,)+ $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            $fields,
            $($tail)*
        }
    };

    (@parse $prefix:tt, $fields:tt, 'a: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'a: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'b: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'b: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'c: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'c: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'd: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'd: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'e: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'e: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'f: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'f: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'g: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'g: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'h: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'h: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'i: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'i: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'j: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'j: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'k: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'k: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'l: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'l: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'm: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'm: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'n: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'n: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'o: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'o: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'p: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'p: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'q: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'q: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'r: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'r: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 's: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 's: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 't: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 't: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'u: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'u: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'v: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'v: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'w: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'w: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'x: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'x: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'y: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'y: [$lt0 $(+ $ltN)*], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'z: $lt0:tt $(+ $ltN:tt)*, $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'z: [$lt0 $(+ $ltN)*], $($tail)* } };

    (@parse $prefix:tt, $fields:tt, 'a: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'a: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'b: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'b: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'c: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'c: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'd: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'd: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'e: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'e: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'f: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'f: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'g: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'g: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'h: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'h: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'i: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'i: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'j: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'j: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'k: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'k: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'l: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'l: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'm: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'm: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'n: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'n: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'o: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'o: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'p: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'p: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'q: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'q: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'r: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'r: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 's: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 's: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 't: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 't: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'u: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'u: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'v: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'v: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'w: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'w: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'x: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'x: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'y: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'y: [$lt0 $(+ $ltN)*], > $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'z: $lt0:tt $(+ $ltN:tt)*> $($tail:tt)*) => { parse_generics_shim! { @app_lt $prefix, $fields, 'z: [$lt0 $(+ $ltN)*], > $($tail)* } };

    (@parse $prefix:tt, $fields:tt, 'a $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'a: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'b $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'b: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'c $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'c: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'd $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'd: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'e $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'e: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'f $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'f: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'g $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'g: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'h $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'h: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'i $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'i: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'j $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'j: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'k $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'k: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'l $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'l: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'm $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'm: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'n $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'n: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'o $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'o: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'p $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'p: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'q $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'q: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'r $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'r: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 's $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 's: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 't $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 't: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'u $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'u: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'v $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'v: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'w $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'w: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'x $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'x: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'y $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'y: [], $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'z $($tail:tt)* ) => { parse_generics_shim! { @app_lt $prefix, $fields, 'z: [], $($tail)* } };

    (
        @app_lt
        $prefix:tt,
        {
            constr: [$($constr:tt)*],
            ltimes: [$($ltimes:tt)*],
            tnames: $tnames:tt,
        },
        $lt:tt: [],
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $lt,],
                ltimes: [$($ltimes)* $lt,],
                tnames: $tnames,
            },
            $($tail)*
        }
    };

    (
        @app_lt
        $prefix:tt,
        {
            constr: [$($constr:tt)*],
            ltimes: [$($ltimes:tt)*],
            tnames: $tnames:tt,
        },
        $lt:tt: [$($ltconstr:tt)*],
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $lt: $($ltconstr)*,],
                ltimes: [$($ltimes)* $lt,],
                tnames: $tnames,
            },
            $($tail)*
        }
    };

    (@parse $prefix:tt, $fields:tt, $tname:ident: 'a $($tail:tt)*) => { parse_generics_shim! { @parse_ty_tail { $prefix, $fields, $tname, }, ['a], $($tail)* } };

    (
        @parse_ty_tail
        {
            $prefix:tt,
            {
                constr: [$($constr:tt)*],
                ltimes: $ltimes:tt,
                tnames: [$($tnames:tt)*],
            },
            $tname:ident,
        },
        [$($tconstrs:tt)*],
        , $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $($tconstrs)*,],
                ltimes: $ltimes,
                tnames: [ $($tnames)* $tname, ],
            },
            $($tail)*
        }
    };

    (
        @parse_ty_tail
        {
            $prefix:tt,
            {
                constr: [$($constr:tt)*],
                ltimes: $ltimes:tt,
                tnames: [$($tnames:tt)*],
            },
            $tname:ident,
        },
        [$($tconstrs:tt)*],
        > $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $($tconstrs)*,],
                ltimes: $ltimes,
                tnames: [ $($tnames)* $tname, ],
            },
            > $($tail)*
        }
    };

    (
        @parse_ty_tail
        {
            $prefix:tt,
            {
                constr: [$($constr:tt)*],
                ltimes: $ltimes:tt,
                tnames: [$($tnames:tt)*],
            },
            $tname:ident,
        },
        [$($tconstrs:tt)*],
        + $tconstr:ty, $($tail)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $($tconstrs)* + $tconstr,],
                ltimes: $ltimes,
                tnames: [ $($tnames)* $tname, ],
            },
            $($tail)*
        }
    };

    (
        @parse_ty_tail
        {
            $prefix:tt,
            {
                constr: [$($constr:tt)*],
                ltimes: $ltimes:tt,
                tnames: [$($tnames:tt)*],
            },
            $tname:ident,
        },
        [$($tconstrs:tt)*],
        + $tconstr:ty> $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $($tconstrs)* + $tconstr,],
                ltimes: $ltimes,
                tnames: [ $($tnames)* $tname, ],
            },
            > $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        {
            constr: [$($constr:tt)*],
            ltimes: $ltimes:tt,
            tnames: [$($tnames:tt)*],
        },
        $tname:ident: $tconstr:ty, $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $tconstr,],
                ltimes: $ltimes,
                tnames: [$($tnames)* $tname,],
            },
            $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        {
            constr: [$($constr:tt)*],
            ltimes: $ltimes:tt,
            tnames: [$($tnames:tt)*],
        },
        $tname:ident: $tconstr:ty> $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname: $tconstr,],
                ltimes: $ltimes,
                tnames: [$($tnames)* $tname,],
            },
            > $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        {
            constr: [$($constr:tt)*],
            ltimes: $ltimes:tt,
            tnames: [$($tnames:tt)*],
        },
        $tname:ident $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @parse
            $prefix,
            {
                constr: [$($constr)* $tname,],
                ltimes: $ltimes,
                tnames: [$($tnames)* $tname,],
            },
            $($tail)*
        }
    };

    (
        @emit_output
        { { .. }, $callback:tt },
        {
            constr: $constr:tt,
            ltimes: [$($ltimes:tt)*],
            tnames: [$($tnames:tt)*],
        },
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @callback
            $callback,
            {
                constr: $constr,
                params: [$($ltimes)* $($tnames)*],
                ltimes: [$($ltimes)*],
                tnames: [$($tnames)*],
                ..
            },
            $($tail)*
        }
    };

    (
        @emit_output
        { { constr, params, ltimes, tnames }, $callback:tt },
        {
            constr: $constr:tt,
            ltimes: [$($ltimes:tt)*],
            tnames: [$($tnames:tt)*],
        },
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @callback
            $callback,
            {
                constr: $constr,
                params: [$($ltimes)* $($tnames)*],
                ltimes: [$($ltimes)*],
                tnames: [$($tnames)*],
            },
            $($tail)*
        }
    };

    (
        @callback
        ($cb_name:ident ! ($($cb_arg:tt)*)),
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @as_expr
            $cb_name!($($cb_arg)* $($tail)*)
        }
    };

    (
        @callback
        ($cb_name:ident ! [$($cb_arg:tt)*]),
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @as_expr
            $cb_name![$($cb_arg)* $($tail)*]
        }
    };

    (
        @callback
        ($cb_name:ident ! {$($cb_arg:tt)*}),
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @as_item
            $cb_name! { $($cb_arg)* $($tail)* }
        }
    };

    (
        $fields:tt,
        then $callback:ident!$callback_arg:tt,
        $($body:tt)*
    ) => {
        parse_generics_shim! {
            @parse_start
            { $fields, ($callback!$callback_arg) },
            $($body)*
        }
    };
}
