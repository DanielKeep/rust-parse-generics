/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate contains a proof-of-concept implementation for the `parse-generics-and-where` RFC.  It requires the `rustc` nightly from 2016-04-06.

As this is a proof-of-concept compiler plugin, you should avoid using this crate *directly*.  Instead, you should use the stable `parse-generics-shim` crate if possible.  The macros in this plugin can be used by enabling the shim crate's `use-parse-generics-poc` feature.

<style type="text/css">
.link-block { font-family: "Fira Sans"; }
.link-block > p { display: inline-block; }
.link-block > p > strong { font-weight: 500; margin-right: 1em; }
.link-block > ul { display: inline-block; padding: 0; list-style: none; }
.link-block > ul > li {
  font-size: 0.8em;
  background-color: #eee;
  border: 1px solid #ccc;
  padding: 0.3em;
  display: inline-block;
}
</style>
<span></span><div class="link-block">

**Links**

* [Latest Release](https://crates.io/crates/parse-generics-poc)
* [Latest Docs](https://danielkeep.github.io/rust-parse-generics/doc/parse_generics_poc/index.html)
* [Repository](https://github.com/DanielKeep/rust-parse-generics)

<span></span></div>

*/
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc_plugin;

use std::borrow::Cow;
use std::convert::Into;
use std::rc::Rc;
use syntax::ast::{self, TokenTree};
use syntax::codemap;
use syntax::errors::DiagnosticBuilder;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::parse::token::str_to_ident;
use syntax::parse::token::{self, DelimToken, Token};
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

macro_rules! delim_tt {
    ({$($e:expr),* $(,)*}) => {
        TokenTree::Delimited(DUM_SP, Rc::new(ast::Delimited {
            delim: DelimToken::Brace,
            open_span: DUM_SP,
            tts: vec![$($e),*],
            close_span: DUM_SP,
        }))
    };

    ([] <- $e:expr) => {
        TokenTree::Delimited(DUM_SP, Rc::new(ast::Delimited {
            delim: DelimToken::Bracket,
            open_span: DUM_SP,
            tts: $e,
            close_span: DUM_SP,
        }))
    };

    ({} <- $e:expr) => {
        TokenTree::Delimited(DUM_SP, Rc::new(ast::Delimited {
            delim: DelimToken::Brace,
            open_span: DUM_SP,
            tts: $e,
            close_span: DUM_SP,
        }))
    };
}

macro_rules! throw {
    ($e:expr) => {
        return ::std::result::Result::Err(::std::convert::From::from($e))
    };
}

macro_rules! handle_field_general {
    ($fields:ident, $tts:ident, $ident_sp:expr, $name:ident, $var:expr) => {
        {
            if $fields.contains(&$var) {
                return Err(Error::span_err($ident_sp,
                    concat!("cannot use `", stringify!($name),
                        "` more than once")));
            }
            $fields.push($var);

            if let Ok(tts_) = can_skip_token(Token::Question, $tts) {
                $tts = tts_;
            }

            if let Ok(tts_) = can_skip_token(Token::Comma, $tts) {
                $tts = tts_;
            } else {
                break;
            }
        }
    };
}

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    reg.register_macro("parse_generics", parse_generics);
    reg.register_macro("parse_where", parse_where);
}

enum Error<'a> {
    Diagnostic(DiagnosticBuilder<'a>),
    SpanErr {
        sp: Option<codemap::Span>,
        msg: Cow<'static, str>,
    },
}

impl<'a> Error<'a> {
    pub fn emit(self, cx: &mut ExtCtxt, fallback_sp: codemap::Span) {
        use self::Error::*;
        match self {
            Diagnostic(mut diag) => diag.emit(),
            SpanErr { sp, msg } => {
                cx.span_err(sp.unwrap_or(fallback_sp), &*msg);
            },
        }
    }

    pub fn diag(diag: DiagnosticBuilder<'a>) -> Error<'a> {
        Error::Diagnostic(diag)
    }

    pub fn err<Str>(msg: Str) -> Error<'static>
    where Str: Into<Cow<'static, str>> {
        Error::SpanErr { sp: None, msg: msg.into() }
    }

    pub fn span_err<Str>(sp: codemap::Span, msg: Str) -> Error<'static>
    where Str: Into<Cow<'static, str>> {
        Error::SpanErr { sp: Some(sp), msg: msg.into() }
    }
}

struct MacMac {
    mac: ast::Mac,
}

impl MacResult for MacMac {
    fn make_expr(self: Box<Self>) -> Option<P<ast::Expr>> {
        let sp = self.mac.span;
        Some(P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprKind::Mac(self.mac),
            span: sp,
            attrs: None,
        }))
    }

    fn make_items(self: Box<Self>) -> Option<SmallVector<P<ast::Item>>> {
        let sp = self.mac.span;
        Some(SmallVector::one(P(ast::Item {
            ident: token::special_idents::invalid,
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Mac(self.mac),
            vis: ast::Visibility::Inherited,
            span: sp,
        })))
    }

    fn make_impl_items(self: Box<Self>) -> Option<SmallVector<ast::ImplItem>> {
        let sp = self.mac.span;
        Some(SmallVector::one(ast::ImplItem {
            id: ast::DUMMY_NODE_ID,
            ident: token::special_idents::invalid,
            vis: ast::Visibility::Inherited,
            defaultness: ast::Defaultness::Final, // FIXME: this is a guess
            attrs: vec![],
            node: ast::ImplItemKind::Macro(self.mac),
            span: sp,
        }))
    }

    fn make_pat(self: Box<Self>) -> Option<P<ast::Pat>> {
        let sp = self.mac.span;
        Some(P(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            node: ast::PatKind::Mac(self.mac),
            span: sp,
        }))
    }

    fn make_stmts(self: Box<Self>) -> Option<SmallVector<ast::Stmt>> {
        let sp = self.mac.span;
        Some(SmallVector::one(codemap::respan(
            sp,
            ast::StmtKind::Mac(
                P(self.mac),
                ast::MacStmtStyle::Braces, // FIXME: this is a guess
                None,
            )
        )))
    }

    fn make_ty(self: Box<Self>) -> Option<P<ast::Ty>> {
        let sp = self.mac.span;
        Some(P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyKind::Mac(self.mac),
            span: sp,
        }))
    }
}

fn parse_generics(
    cx: &mut ExtCtxt,
    sp: codemap::Span,
    tts: &[TokenTree]
) -> Box<MacResult+'static> {
    match try_parse_generics(cx, sp, tts) {
        Ok(res) => res,
        Err(err) => {
            err.emit(cx, sp);
            DummyResult::any(sp)
        }
    }
}

fn parse_where(
    cx: &mut ExtCtxt,
    sp: codemap::Span,
    tts: &[TokenTree]
) -> Box<MacResult+'static> {
    match try_parse_where(cx, sp, tts) {
        Ok(res) => res,
        Err(err) => {
            err.emit(cx, sp);
            DummyResult::any(sp)
        }
    }
}

#[derive(Eq, PartialEq)]
enum GenericField {
    Constr,
    Params,
    LTimes,
    TNames,
}

fn try_parse_generics<'cx>(
    cx: &mut ExtCtxt<'cx>,
    _sp: codemap::Span,
    tts: &[TokenTree]
) -> Result<Box<MacResult + 'static>, Error<'cx>> {
    let mut fields = vec![];

    let (field_tt, tts) = try!(eat_delim_brace(tts));
    let mut field_tts = &field_tt.tts[..];

    let append_ellipsis = if is_just_dotdot(field_tts) {
        fields.push(GenericField::Constr);
        fields.push(GenericField::Params);
        fields.push(GenericField::LTimes);
        fields.push(GenericField::TNames);
        field_tts = &[];
        true
    } else {
        false
    };

    while field_tts.len() > 0 {
        let (ident_sp, ident, field_tts_) = try!(eat_ident(field_tts));
        field_tts = field_tts_;

        let ident = &*ident.name.as_str();

        macro_rules! handle_field {
            ($name:ident, $var:ident) => {
                handle_field_general!(fields, field_tts, ident_sp, $name, GenericField::$var)
            };
        }

        match ident {
            "constr" => handle_field!(constr, Constr),
            "params" => handle_field!(params, Params),
            "ltimes" => handle_field!(ltimes, LTimes),
            "tnames" => handle_field!(tnames, TNames),
            _ => {
                if let Ok(field_tts_) = can_skip_token(Token::Question, field_tts) {
                    // Ignore this ident.
                    field_tts = can_skip_token(Token::Comma, field_tts_).unwrap_or(field_tts_);
                } else {
                    return Err(Error::span_err(ident_sp,
                        format!("unexpected token `{}`", ident)))
                }
            }
        }
    }

    if let Some(ref tt) = field_tts.first() {
        throw!(Error::span_err(tt.get_span(),
            format!("unexpected token `{:?}`", tt)));
    }

    let tts = try!(skip_token(Token::Comma, tts));
    let tts = try!(skip_ident_str("then", tts));
    let (callback_sp, callback, tts) = try!(eat_ident(tts));
    let tts = try!(skip_token(Token::Not, tts));
    let (callback_args, tts) = try!(eat_delim(tts));
    let tts = try!(skip_token(Token::Comma, tts));

    let mut parser = cx.new_parser_from_tts(tts);
    let gen = try!(parser.parse_generics()
        .map_err(Error::diag));

    let tail = try!(parser.parse_all_token_trees()
        .map_err(Error::diag));

    const DUM_SP: codemap::Span = codemap::DUMMY_SP;

    let mut constr = vec![];
    let mut params = vec![];
    let mut ltimes = vec![];
    let mut tnames = vec![];

    for ltime in gen.lifetimes {
        params.push(ltime_tt(ltime.lifetime));
        params.push(tok_tt(Token::Comma));

        ltimes.push(ltime_tt(ltime.lifetime));
        ltimes.push(tok_tt(Token::Comma));

        ltime_def_to_tts(&ltime, &mut constr);
        constr.push(tok_tt(Token::Comma));
    }

    for param in gen.ty_params.move_iter() {
        params.push(ident_tt(param.ident));
        params.push(tok_tt(token::Comma));

        tnames.push(ident_tt(param.ident));
        tnames.push(tok_tt(token::Comma));

        constr.push(ident_tt(param.ident));
        if param.bounds.len() > 0 {
            constr.push(tok_tt(Token::Colon));
            let mut need_plus = false;
            for bound in param.bounds.move_iter() {
                ty_param_bound_to_tts(&bound, &mut constr, &mut need_plus);
            }
        }
        constr.push(tok_tt(Token::Comma));
    }

    let mut ex_tts = callback_args.tts.clone();

    let mut ex_fields = vec![];
    let mut constr = Some(constr);
    let mut params = Some(params);
    let mut ltimes = Some(ltimes);
    let mut tnames = Some(tnames);

    for field in fields {
        match field {
            GenericField::Constr => {
                ex_fields.push(ident_str_tt("constr"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- constr.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
            GenericField::Params => {
                ex_fields.push(ident_str_tt("params"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- params.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
            GenericField::LTimes => {
                ex_fields.push(ident_str_tt("ltimes"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- ltimes.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
            GenericField::TNames => {
                ex_fields.push(ident_str_tt("tnames"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- tnames.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
        }
    }

    if append_ellipsis {
        ex_fields.push(tok_tt(Token::DotDot));
    }

    ex_tts.push(delim_tt!({} <- ex_fields));
    ex_tts.push(tok_tt(Token::Comma));

    {
        let mut tail = tail;
        ex_tts.append(&mut tail);
    }

    let res = Box::new(MacMac {
        mac: codemap::respan(callback_sp, ast::Mac_ {
            path: ast::Path {
                span: callback_sp,
                global: false,
                segments: vec![
                    ast::PathSegment {
                        identifier: *callback,
                        parameters: ast::PathParameters::none(),
                    },
                ],
            },
            tts: ex_tts,
            ctxt: ast::EMPTY_CTXT,
        })
    });

    Ok(res as Box<MacResult + 'static>)
}

#[derive(Eq, PartialEq)]
enum WhereField {
    Clause,
    Preds,
}

fn try_parse_where<'a>(
    cx: &mut ExtCtxt<'a>,
    _sp: codemap::Span,
    tts: &[TokenTree]
) -> Result<Box<MacResult + 'static>, Error<'a>> {
    let mut fields = vec![];

    let (field_tt, tts) = try!(eat_delim_brace(tts));
    let mut field_tts = &field_tt.tts[..];

    let append_ellipsis = if is_just_dotdot(field_tts) {
        fields.push(WhereField::Clause);
        fields.push(WhereField::Preds);
        field_tts = &[];
        true
    } else {
        false
    };

    while field_tts.len() > 0 {
        let (ident_sp, ident, field_tts_) = try!(eat_ident(field_tts));
        field_tts = field_tts_;

        let ident = &*ident.name.as_str();

        macro_rules! handle_field {
            ($name:ident, $var:ident) => {
                handle_field_general!(fields, field_tts, ident_sp, $name, WhereField::$var)
            };
        }

        match ident {
            "clause" => handle_field!(clause, Clause),
            "preds" => handle_field!(preds, Preds),
            _ => {
                if let Ok(field_tts_) = can_skip_token(Token::Question, field_tts) {
                    // Ignore this ident.
                    field_tts = can_skip_token(Token::Comma, field_tts_).unwrap_or(field_tts_);
                } else {
                    return Err(Error::span_err(ident_sp,
                        format!("unexpected token `{}`", ident)))
                }
            }
        }
    }

    if let Some(ref tt) = field_tts.first() {
        throw!(Error::span_err(tt.get_span(),
            format!("unexpected token `{:?}`", tt)));
    }

    let tts = try!(skip_token(Token::Comma, tts));
    let tts = try!(skip_ident_str("then", tts));
    let (callback_sp, callback, tts) = try!(eat_ident(tts));
    let tts = try!(skip_token(Token::Not, tts));
    let (callback_args, tts) = try!(eat_delim(tts));
    let tts = try!(skip_token(Token::Comma, tts));

    let mut parser = cx.new_parser_from_tts(tts);
    let wh = try!(parser.parse_where_clause()
        .map_err(Error::diag));

    let tail = try!(parser.parse_all_token_trees()
        .map_err(Error::diag));

    const DUM_SP: codemap::Span = codemap::DUMMY_SP;

    let mut preds = vec![];

    for pred in &wh.predicates {
        pred_to_tts(pred, &mut preds);
        preds.push(tok_tt(Token::Comma));
    }

    let clause = {
        if preds.len() != 0 {
            let mut clause = Vec::with_capacity(preds.len() + 1);
            clause.push(ident_str_tt("where"));
            clause.extend(preds.iter().cloned());
            clause
        } else {
            vec![]
        }
    };

    let mut ex_tts = callback_args.tts.clone();

    let mut ex_fields = vec![];
    let mut clause = Some(clause);
    let mut preds = Some(preds);

    for field in fields {
        match field {
            WhereField::Clause => {
                ex_fields.push(ident_str_tt("clause"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- clause.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
            WhereField::Preds => {
                ex_fields.push(ident_str_tt("preds"));
                ex_fields.push(tok_tt(Token::Colon));
                ex_fields.push(delim_tt!([] <- preds.take().unwrap()));
                ex_fields.push(tok_tt(Token::Comma));
            },
        }
    }

    if append_ellipsis {
        ex_fields.push(tok_tt(Token::DotDot));
    }

    ex_tts.push(delim_tt!({} <- ex_fields));
    ex_tts.push(tok_tt(Token::Comma));

    {
        let mut tail = tail;
        ex_tts.append(&mut tail);
    }

    let res = Box::new(MacMac {
        mac: codemap::respan(callback_sp, ast::Mac_ {
            path: ast::Path {
                span: callback_sp,
                global: false,
                segments: vec![
                    ast::PathSegment {
                        identifier: *callback,
                        parameters: ast::PathParameters::none(),
                    }
                ],
            },
            tts: ex_tts,
            ctxt: ast::EMPTY_CTXT,
        })
    });

    Ok(res as Box<MacResult + 'static>)
}

fn is_just_dotdot(tts: &[TokenTree]) -> bool {
    if tts.len() != 1 { return false; }
    match tts[0] {
        TokenTree::Token(_, Token::DotDot) => true,
        _ => false,
    }
}

fn eat_delim(tts: &[TokenTree])
-> Result<(&ast::Delimited, &[TokenTree]), Error<'static>> {
    match tts.get(0) {
        Some(&TokenTree::Delimited(_, ref delim)) => Ok((&**delim, &tts[1..])),
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            "expected delimited sequence")),
        None => throw!(Error::err("expected delimited sequence"))
    }
}

fn eat_delim_brace(tts: &[TokenTree])
-> Result<(&ast::Delimited, &[TokenTree]), Error<'static>> {
    match tts.get(0) {
        Some(&TokenTree::Delimited(_, ref delim))
            if delim.delim == DelimToken::Brace => Ok((&**delim, &tts[1..])),
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            "expected brace-delimited sequence")),
        None => throw!(Error::err("expected brace-delimited sequence"))
    }
}

fn eat_ident(tts: &[TokenTree])
-> Result<(codemap::Span, &ast::Ident, &[TokenTree]), Error<'static>> {
    match tts.get(0) {
        Some(&TokenTree::Token(sp, Token::Ident(ref ident, _))) => {
            Ok((sp, ident, &tts[1..]))
        },
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            "expected identifier")),
        None => throw!(Error::err("expected identifier"))
    }
}
 
fn skip_ident_str<'a>(s: &str, tts: &'a [TokenTree])
-> Result<&'a [TokenTree], Error<'static>> {
    match tts.get(0) {
        Some(&TokenTree::Token(_,
            Token::Ident(ast::Ident {
                ref name,
                ctxt: _,
            }, _))) if name.as_str() == s
        => {
            Ok(&tts[1..])
        },
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            format!("expected `{}`", s))),
        None => throw!(Error::err(format!("expected `{}`", s)))
    }
}

fn skip_token(tok: Token, tts: &[TokenTree]) -> Result<&[TokenTree], Error<'static>> {
    match can_skip_token(tok.clone(), tts) {
        Ok(v) => Ok(v),
        Err(Some(tt)) => throw!(Error::span_err(tt.get_span(),
            format!("expected `{:?}`", tok))),
        Err(None) => throw!(Error::err(format!("expected `{:?}`", tok))),
    }
}

fn can_skip_token(tok: Token, tts: &[TokenTree]) -> Result<&[TokenTree], Option<&TokenTree>> {
    match tts.get(0) {
        Some(&TokenTree::Token(_, ref got_tok)) if *got_tok == tok => Ok(&tts[1..]),
        Some(tt) => Err(Some(tt)),
        None => Err(None),
    }
}

fn ident_tt(ident: ast::Ident) -> TokenTree {
    TokenTree::Token(
        codemap::DUMMY_SP,
        Token::Ident(ident, token::IdentStyle::Plain)
    )
}

fn ident_str_tt(s: &str) -> TokenTree {
    TokenTree::Token(
        codemap::DUMMY_SP,
        Token::Ident(str_to_ident(s), token::IdentStyle::Plain)
    )
}

fn ltime_tt(ltime: ast::Lifetime) -> TokenTree {
    TokenTree::Token(
        ltime.span,
        Token::Lifetime(ast::Ident::with_empty_ctxt(ltime.name))
    )
}

fn tok_tt(tok: Token) -> TokenTree {
    TokenTree::Token(codemap::DUMMY_SP, tok)
}

fn ltime_def_to_tts(ltd: &ast::LifetimeDef, tts: &mut Vec<TokenTree>) {
    tts.push(ltime_tt(ltd.lifetime));
    if ltd.bounds.len() > 0 {
        tts.push(tok_tt(Token::Colon));
        let mut need_plus = false;
        for bound in &ltd.bounds {
            if need_plus {
                tts.push(tok_tt(Token::BinOp(token::BinOpToken::Plus)));
            }
            need_plus = true;
            tts.push(ltime_tt(*bound));
        }
    }
}

fn ty_param_bound_to_tts(
    tpb: &ast::TyParamBound,
    tts: &mut Vec<TokenTree>,
    need_plus: &mut bool,
) {
    use syntax::ast::TyParamBound::*;

    macro_rules! emit_plus {
        () => {
            {
                if *need_plus {
                    tts.push(tok_tt(Token::BinOp(token::BinOpToken::Plus)));
                }
                *need_plus = true;
            }
        };
    }

    match *tpb {
        TraitTyParamBound(ref ptr, ref tbm) => {
            emit_plus!();
            match *tbm {
                ast::TraitBoundModifier::None => (),
                ast::TraitBoundModifier::Maybe => {
                    tts.push(tok_tt(Token::Question));
                },
            }
            if ptr.bound_lifetimes.len() > 0 {
                tts.push(ident_str_tt("for"));
                tts.push(tok_tt(Token::Lt));
                let mut need_comma = false;
                for ltd in &ptr.bound_lifetimes {
                    if need_comma {
                        tts.push(tok_tt(Token::Comma));
                    }
                    need_comma = true;
                    ltime_def_to_tts(ltd, tts);
                }
                tts.push(tok_tt(Token::Gt));
            }
            path_to_tts(&ptr.trait_ref.path, tts);
        },
        RegionTyParamBound(lt) => {
            emit_plus!();
            tts.push(ltime_tt(lt));
        },
    }
}

fn path_to_tts(path: &ast::Path, tts: &mut Vec<TokenTree>) {
    use syntax::ast::PathParameters as PP;
    if path.global {
        tts.push(tok_tt(Token::ModSep));
    }
    for (i, seg) in (&path.segments).into_iter().enumerate() {
        if i > 0 {
            tts.push(tok_tt(Token::ModSep));
        }
        tts.push(ident_tt(seg.identifier));

        // Use separating commas, since the shim has to blind-parse paths,
        // and can't really insert terminating commas.
        macro_rules! maybe_comma {
            ($need_comma:ident) => {
                if $need_comma { tts.push(tok_tt(Token::Comma)); }
                $need_comma = true;
            };
        }

        match seg.parameters {
            PP::AngleBracketed(ref data) if !seg.parameters.is_empty() => {
                let mut need_comma = false;
                tts.push(tok_tt(Token::Lt));
                for lifetime in &data.lifetimes {
                    maybe_comma!(need_comma);
                    tts.push(ltime_tt(*lifetime));
                }
                for ty in &data.types {
                    maybe_comma!(need_comma);
                    tts.push(nt_ty_tt(ty.clone()));
                }
                for binding in &data.bindings {
                    maybe_comma!(need_comma);
                    tts.push(ident_tt(binding.ident));
                    tts.push(tok_tt(Token::Eq));
                    tts.push(nt_ty_tt(binding.ty.clone()));
                }
                tts.push(tok_tt(Token::Gt));
            },
            PP::AngleBracketed(_) => (),
            PP::Parenthesized(ref data) => {
                let mut need_comma = false;
                tts.push(tok_tt(Token::OpenDelim(DelimToken::Paren)));
                for input in &data.inputs {
                    maybe_comma!(need_comma);
                    tts.push(nt_ty_tt(input.clone()));
                }
                tts.push(tok_tt(Token::CloseDelim(DelimToken::Paren)));
                if let Some(ref output) = data.output {
                    tts.push(tok_tt(Token::RArrow));
                    tts.push(nt_ty_tt(output.clone()));
                }
            },
        }
    }
}

fn nt_ty_tt(ty: P<ast::Ty>) -> TokenTree {
    tok_tt(Token::Interpolated(token::Nonterminal::NtTy(ty)))
}

fn pred_to_tts(pred: &ast::WherePredicate, tts: &mut Vec<TokenTree>) {
    use syntax::ast::WherePredicate as WP;
    match *pred {
        WP::BoundPredicate(ref wbp) => {
            if wbp.bound_lifetimes.len() > 0 {
                tts.push(ident_str_tt("for"));
                tts.push(tok_tt(Token::Lt));
                for ltd in &wbp.bound_lifetimes {
                    // Use terminating commas here, as it's easier for the shim
                    // to construct.
                    ltime_def_to_tts(ltd, tts);
                    tts.push(tok_tt(Token::Comma));
                }
                tts.push(tok_tt(Token::Gt));
            }
            tts.push(nt_ty_tt(wbp.bounded_ty.clone()));
            tts.push(tok_tt(Token::Colon));
            let mut need_plus = false;
            for tpb in &wbp.bounds {
                ty_param_bound_to_tts(tpb, tts, &mut need_plus);
            }
        },
        WP::RegionPredicate(ref rp) => {
            tts.push(ltime_tt(rp.lifetime));
            tts.push(tok_tt(Token::Colon));
            let mut need_plus = false;
            for bound in &rp.bounds {
                if need_plus {
                    tts.push(tok_tt(Token::BinOp(token::BinOpToken::Plus)));
                }
                need_plus = true;
                tts.push(ltime_tt(*bound));
            }
        },
        WP::EqPredicate(_) => {
            panic!("equality predicates should not exist");
        },
    }
}
