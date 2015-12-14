#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc_plugin;

use std::borrow::Cow;
use std::convert::Into;
use std::rc::Rc;
use syntax::ast::{self, TokenTree};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::parse::token::str_to_ident;
use syntax::parse::token::{self, Token};
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

macro_rules! throw {
    ($e:expr) => {
        return ::std::result::Result::Err(::std::convert::From::from($e))
    };
}

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    reg.register_macro("parse_generics", parse_generics);
}

enum Error {
    SpanErr {
        sp: Option<codemap::Span>,
        msg: Cow<'static, str>,
    },
}

impl Error {
    pub fn err<Str>(msg: Str) -> Error
    where Str: Into<Cow<'static, str>> {
        Error::SpanErr { sp: None, msg: msg.into() }
    }

    pub fn span_err<Str>(sp: codemap::Span, msg: Str) -> Error
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
            node: ast::ExprMac(self.mac),
            span: sp,
            attrs: None,
        }))
    }

    fn make_items(self: Box<Self>) -> Option<SmallVector<P<ast::Item>>> {
        let sp = self.mac.span;
        Some(SmallVector::one(P(ast::Item {
            ident: str_to_ident("callback"),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemMac(self.mac),
            vis: ast::Visibility::Inherited,
            span: sp,
        })))
    }

    fn make_impl_items(self: Box<Self>) -> Option<SmallVector<P<ast::ImplItem>>> {
        let sp = self.mac.span;
        Some(SmallVector::one(P(ast::ImplItem {
            id: ast::DUMMY_NODE_ID,
            ident: str_to_ident("callback"),
            vis: ast::Visibility::Inherited,
            attrs: vec![],
            node: ast::ImplItemKind::Macro(self.mac),
            span: sp,
        })))
    }

    fn make_pat(self: Box<Self>) -> Option<P<ast::Pat>> {
        let sp = self.mac.span;
        Some(P(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            node: ast::PatMac(self.mac),
            span: sp,
        }))
    }

    fn make_stmts(self: Box<Self>) -> Option<SmallVector<P<ast::Stmt>>> {
        let sp = self.mac.span;
        Some(SmallVector::one(P(codemap::respan(
            sp,
            ast::StmtMac(
                P(self.mac),
                ast::MacStmtStyle::MacStmtWithBraces, // FIXME: this is a guess
                None,
            )
        ))))
    }

    fn make_ty(self: Box<Self>) -> Option<P<ast::Ty>> {
        let sp = self.mac.span;
        Some(P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyMac(self.mac),
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
            use Error::*;
            match err {
                SpanErr { sp: err_sp, msg } => {
                    cx.span_err(err_sp.unwrap_or(sp), &*msg);
                },
            }
            DummyResult::any(sp)
        }
    }
}

fn try_parse_generics(
    cx: &mut ExtCtxt,
    _sp: codemap::Span,
    tts: &[TokenTree]
) -> Result<Box<MacResult + 'static>, Error> {
    let tts = try!(skip_ident_str("then", tts));
    let (callback_sp, callback, tts) = try!(eat_ident(tts));
    let tts = try!(skip_token(Token::Not, tts));
    let (_args, tts) = try!(eat_delim(tts));
    let tts = try!(skip_token(Token::Comma, tts));

    let mut parser = cx.new_parser_from_tts(tts);
    let gen = try!(parser.parse_generics()
        .map_err(|_| Error::SpanErr {
            sp: tts.get(0).map(|tt| tt.get_span()),
            msg: "expected generic parameters".into(),
        }));

    let tail = try!(parser.parse_all_token_trees()
        .map_err(|_| Error::SpanErr {
            sp: Some(parser.span),
            msg: "somehow, could not parse tts".into(),
        }));

    const DUM_SP: codemap::Span = codemap::DUMMY_SP;

    macro_rules! delim_tt {
        ({$($e:expr),* $(,)*}) => {
            TokenTree::Delimited(DUM_SP, Rc::new(ast::Delimited {
                delim: token::DelimToken::Brace,
                open_span: DUM_SP,
                tts: vec![$($e),*],
                close_span: DUM_SP,
            }))
        };

        ([] <- $e:expr) => {
            TokenTree::Delimited(DUM_SP, Rc::new(ast::Delimited {
                delim: token::DelimToken::Bracket,
                open_span: DUM_SP,
                tts: $e,
                close_span: DUM_SP,
            }))
        };
    }

    let mut ltimes = vec![];
    let mut params = vec![];
    let mut constr = vec![];

    for ltime in gen.lifetimes {
        ltimes.push(ltime_tt(ltime.lifetime));
        ltimes.push(tok_tt(Token::Comma));

        ltime_def_to_tts(&ltime, &mut constr);
        constr.push(tok_tt(Token::Comma));
    }

    for param in gen.ty_params.move_iter() {
        params.push(ident_tt(param.ident));
        params.push(tok_tt(token::Comma));

        constr.push(ident_tt(param.ident));
        if param.bounds.len() > 0 {
            constr.push(tok_tt(Token::Colon));
            let mut need_plus = false;
            for bound in param.bounds.move_iter() {
                try!(ty_param_bound_to_tts(&bound, &mut constr, &mut need_plus));
            }
        }
        constr.push(tok_tt(Token::Comma));
    }

    let mut ex_tts = vec![
        delim_tt!({
            ident_str_tt("constr"),
            tok_tt(Token::Colon),
            delim_tt!([] <- constr),
            tok_tt(Token::Comma),

            ident_str_tt("ltimes"),
            tok_tt(Token::Colon),
            delim_tt!([] <- ltimes),
            tok_tt(Token::Comma),

            ident_str_tt("params"),
            tok_tt(Token::Colon),
            delim_tt!([] <- params),
        }),
        tok_tt(Token::Comma),
    ];

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

fn eat_delim(tts: &[TokenTree])
-> Result<(&ast::Delimited, &[TokenTree]), Error> {
    match tts.get(0) {
        Some(&TokenTree::Delimited(_, ref delim)) => Ok((&**delim, &tts[1..])),
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            "expected delimited sequence")),
        None => throw!(Error::err("expected delimited sequence"))
    }
}

fn eat_ident(tts: &[TokenTree])
-> Result<(codemap::Span, &ast::Ident, &[TokenTree]), Error> {
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
-> Result<&'a [TokenTree], Error> {
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

fn skip_token(tok: Token, tts: &[TokenTree]) -> Result<&[TokenTree], Error> {
    match tts.get(0) {
        Some(&TokenTree::Token(_, ref got_tok)) if *got_tok == tok
        => Ok(&tts[1..]),
        Some(&ref tt) => throw!(Error::span_err(tt.get_span(),
            format!("expected `{:?}`", tok))),
        None => throw!(Error::err(format!("expected `{:?}`", tok)))
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
) -> Result<(), Error> {
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
            tts.push(tok_tt(Token::Interpolated(
                token::Nonterminal::NtPath(
                    Box::new(ptr.trait_ref.path.clone())))));
        },
        RegionTyParamBound(lt) => {
            emit_plus!();
            tts.push(ltime_tt(lt));
        },
    }
    Ok(())
}
