#![forbid(unsafe_code)]
#![allow(bad_style)]

extern crate proc_macro;

use ::proc_macro::{*,
    Span,
    TokenStream as TS,
    TokenTree as TT,
};

#[macro_use]
mod utils;

#[proc_macro_attribute] pub
fn dyn_safe (attrs: TS, input: TS) -> TS
{
    match dyn_safe_impl(attrs, input) {
        | Ok(it) => {
            // println!("{}", it.to_string());
            it
        },
        | Err((
            (start_span, mb_end_span),
            err_msg,
        )) => {
            macro_rules! spanned {($expr:expr) => (
                match $expr { mut expr => {
                    expr.set_span(start_span);
                    expr
                }}
            )}
            return ts![
                Ident::new("compile_error", start_span),
                spanned!(Punct::new('!', Spacing::Alone)),
                spanned!(ts![ (
                    Literal::string(&*err_msg),
                )]),
                {
                    let mut it = spanned!(Punct::new(';', Spacing::Alone));
                    if let Some(end_span) = mb_end_span {
                        it.set_span(end_span);
                    }
                    it
                },
            ];
        },
    }
}

fn dyn_safe_impl (attrs: TS, input: TS)
  -> Result<
        TS,
        (
            (Span, Option<Span>),
            &'static str, // impl 'static + ::core::ops::Deref<Target = str>,
        ),
    >
{Ok({
    let mut attrs = attrs.into_iter();
    let mut input = input.into_iter().peekable();
    let s;
    let (dyn_safe, dyn_safe_span) = unwrap_next_token! {
        if let TT::Ident(ident) = attrs.next(),
            if ({
                s = ident.to_string();
                s == "true" || s == "false"
            })
        {
            (s == "true", ident.span())
        } else {
            failwith!("expected `true`");
        }
    };
    // End of attrs.
    match attrs.next() {
        | None => {}, // OK
        // | Some(TT::Ident(where_)) if where_.to_string() == "where" => {
        //     // Easter egg path
        //     match (attrs.next(), attrs.next(), attrs.next()) {
        //         | (
        //             Some(TT::Ident(Self_)),
        //             Some(TT::Punct(colon)),
        //             Some(TT::Ident(Sized_)),
        //         )   if Self_.to_string() == "Self"
        //             && colon.as_char() == ':'
        //             && Sized_.to_string() == "Sized"
        //         => error! {
        //             "👏👏👏👏👏👏👏👏👏👏👏👏👏👏👏👏👏👏👏"
        //                 => where_.span(), Sized_.span()
        //         },
        //         | _ => error! {
        //             "extraneous token(s)" => where_.span(),
        //         },
        //     }
        // },
        | Some(extraneous_tt) => error! {
            "extraneous token(s)" => extraneous_tt.span(),
        },
    }

    let mut prefix = vec![];
    while matches!(input.peek(),
        Some(TT::Punct(p)) if p.as_char() == '#'
    )
    {
        prefix.extend(input.by_ref().take(2));
    }
    if matches!(input.peek(),
        Some(TT::Ident(pub_)) if pub_.to_string() == "pub"
    )
    {
        prefix.push(input.next().unwrap());
        if matches!(input.peek(),
            Some(TT::Group(g)) if g.delimiter() == Delimiter::Parenthesis
        )
        {
            prefix.push(input.next().unwrap());
        }
    }
    if matches!(input.peek(),
        Some(TT::Ident(unsafe_)) if unsafe_.to_string() == "unsafe"
    )
    {
        prefix.push(input.next().unwrap());
    }
    if matches!(input.peek(),
        Some(TT::Ident(auto_)) if auto_.to_string() == "auto"
    )
    {
        prefix.push(input.next().unwrap());
    }
    unwrap_next_token! {
        if let TT::Ident(trait_) = input.next(),
            if (trait_.to_string() == "trait")
        {
            prefix.push(trait_.into());
        } else {
            failwith!("expected `trait`");
        }
    }
    let trait_name = unwrap_next_token! {
        if let TT::Ident(ident) = input.next(), { ident } else {
            failwith!("expected an identifier");
        }
    };
    if dyn_safe {
        return Ok(
            prefix
                .into_iter()
                .chain(Some(trait_name.clone().into()))
                .chain(input)
                .chain(ts![
                    Ident::new("impl", dyn_safe_span),
                    Ident::new("dyn", dyn_safe_span),
                    trait_name,
                    ts![{}],
                ])
                .collect()
        );
    }
    prefix.push(trait_name.into());
    // Generics
    let mut depth = 0;
    loop {
        match input.peek() {
            | Some(TT::Punct(p)) if p.as_char() == '<' => {
                prefix.push(input.next().unwrap());
                depth += 1;
            },
            | Some(TT::Punct(p)) if p.as_char() == '>' => {
                prefix.push(input.next().unwrap());
                depth -= 1;
            },
            | Some(_) if depth != 0 => {
                prefix.push(input.next().unwrap());
            },
            | Some(TT::Punct(p)) if p.as_char() == ':' => {
                prefix.push(input.next().unwrap());
                break;
            },
            | _ => {
                prefix.push(Punct::new(':', Spacing::Alone).into());
                break;
            },
        }
    }
    prefix
        .into_iter()
        .chain(ts![
            Punct::new(':', Spacing::Joint),
            Punct::new(':', Spacing::Alone),
            Ident::new("dyn_safe", dyn_safe_span),
            Punct::new(':', Spacing::Joint),
            Punct::new(':', Spacing::Alone),
            Ident::new("NotObjectSafe", dyn_safe_span),
            Punct::new('+', Spacing::Alone),
        ])
        .chain(input)
        .collect()
})}
