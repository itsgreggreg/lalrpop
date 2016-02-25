#![allow(unused_imports)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_S<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<i32, __ParseError<(),Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __lookbehind: () = ::std::default::Default::default();
        match try!(__state0(__lookbehind, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____S((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        E(((), i32, ())),
        S(((), i32, ())),
        T(((), i32, ())),
        ____S(((), i32, ())),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state4(__lookbehind, __tokens, __sym0));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(__lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::S(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(__lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state6(__lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(__sym0, &__start, &__end);
                let __nt = __Nonterminal::S((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0, &__start, &__end);
                let __nt = __Nonterminal::____S((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(__sym0, &__start, &__end);
                let __nt = __Nonterminal::E((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(__lookbehind, __tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(__lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(__sym0, &__start, &__end);
                let __nt = __Nonterminal::T((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state4(__lookbehind, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(__lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(__lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state12(__lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state13(__lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(__sym0, &__start, &__end);
                let __nt = __Nonterminal::E((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(__lookbehind, __tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(__lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(__sym0, &__start, &__end);
                let __nt = __Nonterminal::T((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(__sym0, __sym1, __sym2, &__start, &__end);
                let __nt = __Nonterminal::E((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: &mut Option<((), i32, ())>,
        __sym2: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2, &__start, &__end);
                let __nt = __Nonterminal::T((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(__lookbehind, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(__lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(__lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(__lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __lookbehind = __loc2.clone();
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state13(__lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: &mut Option<((), i32, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(__sym0, __sym1, __sym2, &__start, &__end);
                let __nt = __Nonterminal::E((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state16<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: &mut Option<((), i32, ())>,
        __sym2: &mut Option<((), Tok, ())>,
    ) -> Result<((), Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: ((), Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2, &__start, &__end);
                let __nt = __Nonterminal::T((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookbehind, __lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__S::parse_S;

pub fn __action0<
>(
    (_, __0, _): ((), i32, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    (_, __0, _): ((), i32, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    (__0)
}

pub fn __action2<
>(
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    l - r
}

pub fn __action3<
>(
    (_, t, _): ((), i32, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    t - super::ZERO
}

pub fn __action4<
>(
    (_, __0, _): ((), i32, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    (__0)
}

pub fn __action5<
>(
    (_, _, _): ((), Tok, ()),
    (_, __0, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    __lookbehind: &(),
    __lookahead: &(),
) -> i32
{
    (__0)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),Tok,()),Self::Error>;
}

impl<> __ToTriple<> for Tok {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(Tok),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        value.map(|v| ((), v, ()))
    }
}
