/// mini-`syn`
macro_rules! unwrap_next_token {(
    if let $($pat:pat)|+ = $mb_token:expr,
        $(if ($($guard:tt)*))?
        $expr:block
    else {
        failwith!($err_msg:expr $(, $($($rest:tt)+)?)? );
    }
) => (
    match $mb_token {
        $( | Some($pat) )+
        $(if $($guard)*)?
        => $expr,
        | Some(bad_token) => return Err((
            (bad_token.span(), None),
            {
                #[allow(unused_macros)]
                macro_rules! it {() => ($err_msg)}
                {
                    $($(
                        macro_rules! it {() => (
                            format!($err_msg, $($rest)+)
                        )}
                    )?)?
                    it!().into()
                }
            },
        )),
        | None => return Err((
            (Span::call_site(), None),
            {
                #[allow(unused_macros)]
                macro_rules! it {() => (
                    concat!("Unexpected end of input: ", $err_msg)
                )}
                {
                    $($(
                        macro_rules! it {() => (
                            format!(
                                concat!("Unexpected end of input: ", $err_msg),
                                $($rest)+
                            )
                        )}
                    )?)?
                    it!().into()
                }
            },
        ))
    }
)}

/// mini-`quote`
macro_rules! ts {
    ( ($($input:tt)*) ) => (
        Group::new(Delimiter::Parenthesis, ts![$($input)*])
    );
    ( [$($input:tt)*] ) => (
        Group::new(Delimiter::Bracket, ts![$($input)*])
    );
    ( {$($input:tt)*} ) => (
        Group::new(Delimiter::Brace, ts![$($input)*])
    );

    (
        $($expr:expr),* $(,)?
    ) => (
        <TokenStream as ::core::iter::FromIterator<TT>>::from_iter(vec![
            $($expr.into() ,)*
        ])
    );
}

macro_rules! ite {(
    ($($something:tt)+) $($ignored:tt)?
) => (
    $($something)*
)}

macro_rules! error {(
    $err_msg:expr $(
        => $span:expr $(, $end_span:expr)?
    )? $(,)?
) => (
    return Err((
        (
            ite! {
                $((
                    $span
                ))? (
                    Span::call_site()
                )
            },
            ite! {
                $($((
                    Some($end_span)
                ))?)? (
                    None
                )
            },
        ),
        $err_msg
    ))
)}
