#[macro_export]
macro_rules! parse_where_shim {
    (
        @parse
        $prefix:tt, $fields:tt,
        ; $($tail:tt)*
    ) => {
        parse_where_shim! {
            @emit_output
            $prefix, $fields,
            ; $($tail)*
        }
    };

    (
        @parse
        $prefix:tt, $fields:tt,
        = $($tail:tt)*
    ) => {
        parse_where_shim! {
            @emit_output
            $prefix, $fields,
            = $($tail)*
        }
    };

    (
        @parse
        $prefix:tt, $fields:tt,
        {$($delim:tt)*} $($tail:tt)*
    ) => {
        parse_where_shim! {
            @emit_output
            $prefix, $fields,
            {$($delim)*} $($tail)*
        }
    };

    (
        @parse
        $prefix:tt, $fields:tt,
        $(,)+ $($tail:tt)*
    ) => {
        parse_where_shim! {
            @parse
            $prefix, $fields,
            $($tail)*
        }
    };

    (@parse $prefix:tt, $fields:tt, 'a: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'a}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'b: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'b}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'c: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'c}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'd: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'d}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'e: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'e}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'f: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'f}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'g: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'g}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'h: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'h}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'i: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'i}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'j: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'j}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'k: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'k}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'l: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'l}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'm: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'m}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'n: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'n}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'o: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'o}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'p: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'p}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'q: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'q}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'r: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'r}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 's: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'s}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 't: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'t}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'u: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'u}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'v: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'v}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'w: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'w}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'x: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'x}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'y: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'y}:  }, $($tail)* } };
    (@parse $prefix:tt, $fields:tt, 'z: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, {'z}:  }, $($tail)* } };

    (
        @parse
        $prefix:tt,
        $fields:tt,
        for $($tail:tt)*
    ) => {
        parse_generics_shim! {
            { constr },
            then parse_where_shim! { @parsed_for $prefix, $fields, },
            $($tail)*
        }
    };

    (
        @parse
        $prefix:tt,
        $fields:tt,
        $tname:ident: $($tail:tt)*
    ) => {
        parse_constr! {
            (true, true),
            then parse_where_shim! { @app_con $prefix, $fields, {$tname}: },
            $($tail)*
        }
    };

    (
        @app_con
        $prefix:tt,
        { preds: [$($preds:tt)*], },
        {$($thing:tt)*}: {},
        $($body:tt)*
    ) => {
        parse_where_shim! {
            @parse
            $prefix,
            { preds: [$($preds)* $($thing)*,], },
            $($body)*
        }
    };

    (
        @app_con
        $prefix:tt,
        { preds: [$($preds:tt)*], },
        {$($thing:tt)*}: {$($constr:tt)*},
        $($body:tt)*
    ) => {
        parse_where_shim! {
            @parse
            $prefix,
            { preds: [$($preds)* $($thing)*: $($constr)*,], },
            $($body)*
        }
    };

    (
        @parsed_for
        $prefix:tt,
        $fields:tt,
        { constr: [$($constr:tt)*], },
        $tname:ident: $($tail:tt)*
    ) => {
        parse_constr! {
            (true, true),
            then parse_where_shim! { @app_con $prefix, $fields, {for<$($constr)*> $tname}: },
            $($tail)*
        }
    };

    (
        @emit_output
        { { .. }, $callback:tt },
        {
            preds: $preds:tt,
        },
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @callback
            $callback,
            {
                preds: $preds,
                ..
            },
            $($tail)*
        }
    };

    (
        @emit_output
        { { preds }, $callback:tt },
        $fields:tt,
        $($tail:tt)*
    ) => {
        parse_generics_shim! {
            @callback
            $callback,
            $fields,
            $($tail)*
        }
    };

    (
        $fields:tt,
        then $callback:ident!$callback_arg:tt,
        where $($body:tt)*
    ) => {
        parse_where_shim! {
            @parse
            { $fields, ($callback!$callback_arg) },
            { preds: [], },
            $($body)*
        }
    };

    (
        $fields:tt,
        then $callback:ident!$callback_arg:tt,
        $($body:tt)*
    ) => {
        parse_where_shim! {
            @emit_output
            { $fields, ($callback!$callback_arg) },
            { preds: [], },
            $($body)*
        }
    };
}
