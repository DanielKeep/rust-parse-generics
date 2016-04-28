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

    (@parse $prefix:tt, $fields:tt, 'a: $($tail:tt)*) => { parse_constr! { (true, false), then parse_where_shim! { @app_con $prefix, $fields, 'a:  }, $($tail)* } };

    // @parse for for<..> ..

    (
        @parse
        $prefix:tt,
        $fields:tt,
        $tname:ident: $($tail:tt)*
    ) => {
        parse_constr! {
            (true, true),
            then parse_where_shim! { @app_con $prefix, $fields, $tname:  },
            $($tail)*
        }
    };

    (
        @app_con
        $prefix:tt,
        { preds: [$($preds:tt)*], },
        $thing:tt: {},
        $($body:tt)*
    ) => {
        parse_where_shim! {
            @parse
            $prefix,
            { preds: [$($preds)* $thing,], },
            $($body)*
        }
    };

    (
        @app_con
        $prefix:tt,
        { preds: [$($preds:tt)*], },
        $thing:tt: {$($constr:tt)*},
        $($body:tt)*
    ) => {
        parse_where_shim! {
            @parse
            $prefix,
            { preds: [$($preds)* $thing: $($constr)*,], },
            $($body)*
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
