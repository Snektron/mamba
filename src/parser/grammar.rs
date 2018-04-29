// auto-generated: "lalrpop 0.15.1"
use ast::{Program, TransitionArm, Pattern};
use dtm::{State, TapeSymbol, Symbol, Move, InputAlphabet};
use utility::MultiMap;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{Program, TransitionArm, Pattern};
    use dtm::{State, TapeSymbol, Symbol, Move, InputAlphabet};
    use utility::MultiMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Symbol),
        Variant2(::std::vec::Vec<Symbol>),
        Variant3(State),
        Variant4(InputAlphabet),
        Variant5(Vec<TransitionArm>),
        Variant6(String),
        Variant7(Vec<Symbol>),
        Variant8(Move),
        Variant9(Pattern),
        Variant10(Program),
        Variant11(TransitionArm),
        Variant12(::std::vec::Vec<TransitionArm>),
        Variant13(TapeSymbol),
        Variant14((State, Vec<TransitionArm>)),
        Variant15(::std::vec::Vec<(State, Vec<TransitionArm>)>),
        Variant16((Vec<State>, MultiMap<State, TransitionArm>)),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 8
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 11
        -23, 0, -23, -23, 0, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 25, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -9, -9, -9, -9, 0, -9, -9, -9, -9, 0, 0, 0, -9, 0, -9,
        // State 15
        0, -22, 0, 0, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, -22,
        // State 16
        0, 32, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 34, 0, 15,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 21
        0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 38, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 26
        0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 28
        -24, 0, 0, -24, 0, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        -16, 0, 0, -16, 0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28,
        // State 31
        -15, 0, 0, -15, 0, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -25, 0, 0, -25, 0, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 32, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 15,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 35
        0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        // State 39
        0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, -20, -20,
        // State 40
        0, 32, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 45, 15,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18,
        // State 42
        0, 32, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 15,
        // State 43
        0, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, -21, -21,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8,
        // State 45
        48, 0, 0, 0, 0, 0, 49, 50, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -19, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, -19, -19,
        // State 47
        0, -13, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, -13, -13,
        // State 48
        0, -12, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -12, -12,
        // State 49
        0, -14, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, -14, -14,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -32,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -29,
        // State 18
        -31,
        // State 19
        -17,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -27,
        // State 26
        0,
        // State 27
        -26,
        // State 28
        0,
        // State 29
        0,
        // State 30
        -28,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -30,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -8,
        // State 45
        0,
        // State 46
        -19,
        // State 47
        -13,
        // State 48
        -12,
        // State 49
        -14,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 11, 0, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 18, 19, 20, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 26, 12, 0, 0, 27, 0, 0, 28, 0, 0, 29, 30, 31, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 35, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 12, 0, 0, 27, 0, 0, 40, 41, 0, 29, 30, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 12, 0, 0, 27, 0, 0, 44, 0, 0, 29, 30, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 12, 0, 0, 46, 0, 0, 0, 0, 0, 29, 30, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""*""###,
            r###"",""###,
            r###""->""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###"">""###,
            r###""_""###,
            r###""accept""###,
            r###""alphabet""###,
            r###""reject""###,
            r###""{""###,
            r###""}""###,
            r###"r#"[0-9A-Za-z]+"#"###,
        ];
        __ACTION[(__state * 15)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ProgramParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Program, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(1, _) if true => 0,
                    Token(2, _) if true => 1,
                    Token(3, _) if true => 2,
                    Token(4, _) if true => 3,
                    Token(5, _) if true => 4,
                    Token(6, _) if true => 5,
                    Token(7, _) if true => 6,
                    Token(8, _) if true => 7,
                    Token(9, _) if true => 8,
                    Token(10, _) if true => 9,
                    Token(11, _) if true => 10,
                    Token(12, _) if true => 11,
                    Token(13, _) if true => 12,
                    Token(14, _) if true => 13,
                    Token(0, _) if true => 14,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 15 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                (|| {
                    // (<Symbol> ",") = Symbol, "," => ActionFn(28);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action28::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 0)
                })()
            }
            2 => {
                (|| {
                    // (<Symbol> ",")* =  => ActionFn(26);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action26::<>(input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 1)
                })()
            }
            3 => {
                (|| {
                    // (<Symbol> ",")* = (<Symbol> ",")+ => ActionFn(27);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action27::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 1)
                })()
            }
            4 => {
                (|| {
                    // (<Symbol> ",")+ = Symbol, "," => ActionFn(31);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action31::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 2)
                })()
            }
            5 => {
                (|| {
                    // (<Symbol> ",")+ = (<Symbol> ",")+, Symbol, "," => ActionFn(32);
                    let __sym2 = __pop_Variant0(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action32::<>(input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (3, __symbol, 2)
                })()
            }
            6 => {
                (|| {
                    // Accept = "accept", ":", State, ";" => ActionFn(9);
                    let __sym3 = __pop_Variant0(__symbols);
                    let __sym2 = __pop_Variant3(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (4, __symbol, 3)
                })()
            }
            7 => {
                (|| {
                    // Alphabet = "alphabet", ":", List<Symbol>, ";" => ActionFn(8);
                    let __sym3 = __pop_Variant0(__symbols);
                    let __sym2 = __pop_Variant7(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (4, __symbol, 4)
                })()
            }
            8 => {
                (|| {
                    // BlockTransArm = "{", ShortTransArm+, "}" => ActionFn(6);
                    let __sym2 = __pop_Variant0(__symbols);
                    let __sym1 = __pop_Variant12(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (3, __symbol, 5)
                })()
            }
            9 => {
                (|| {
                    // Ident = r#"[0-9A-Za-z]+"# => ActionFn(17);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action17::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (1, __symbol, 6)
                })()
            }
            10 => {
                (|| {
                    // List<Symbol> = Symbol => ActionFn(33);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action33::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                    (1, __symbol, 7)
                })()
            }
            11 => {
                (|| {
                    // List<Symbol> = (<Symbol> ",")+, Symbol => ActionFn(34);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action34::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                    (2, __symbol, 7)
                })()
            }
            12 => {
                (|| {
                    // Move = "<" => ActionFn(18);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action18::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            13 => {
                (|| {
                    // Move = "!" => ActionFn(19);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action19::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            14 => {
                (|| {
                    // Move = ">" => ActionFn(20);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action20::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            15 => {
                (|| {
                    // Pattern = "*" => ActionFn(11);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action11::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                    (1, __symbol, 9)
                })()
            }
            16 => {
                (|| {
                    // Pattern = TapeSymbol => ActionFn(12);
                    let __sym0 = __pop_Variant13(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action12::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                    (1, __symbol, 9)
                })()
            }
            17 => {
                (|| {
                    // Program = Alphabet, Accept, Reject, Transitions => ActionFn(1);
                    let __sym3 = __pop_Variant16(__symbols);
                    let __sym2 = __pop_Variant3(__symbols);
                    let __sym1 = __pop_Variant3(__symbols);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                    (4, __symbol, 10)
                })()
            }
            18 => {
                (|| {
                    // Reject = "reject", ":", State, ";" => ActionFn(10);
                    let __sym3 = __pop_Variant0(__symbols);
                    let __sym2 = __pop_Variant3(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (4, __symbol, 11)
                })()
            }
            19 => {
                (|| {
                    // ShortTransArm = Pattern, "->", State, Pattern, Move => ActionFn(7);
                    let __sym4 = __pop_Variant8(__symbols);
                    let __sym3 = __pop_Variant9(__symbols);
                    let __sym2 = __pop_Variant3(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant9(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym4.2.clone();
                    let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                    let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                    (5, __symbol, 12)
                })()
            }
            20 => {
                (|| {
                    // ShortTransArm+ = ShortTransArm => ActionFn(22);
                    let __sym0 = __pop_Variant11(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action22::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                    (1, __symbol, 13)
                })()
            }
            21 => {
                (|| {
                    // ShortTransArm+ = ShortTransArm+, ShortTransArm => ActionFn(23);
                    let __sym1 = __pop_Variant11(__symbols);
                    let __sym0 = __pop_Variant12(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action23::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                    (2, __symbol, 13)
                })()
            }
            22 => {
                (|| {
                    // State = Ident => ActionFn(16);
                    let __sym0 = __pop_Variant6(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action16::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (1, __symbol, 14)
                })()
            }
            23 => {
                (|| {
                    // Symbol = Ident => ActionFn(15);
                    let __sym0 = __pop_Variant6(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action15::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            24 => {
                (|| {
                    // TapeSymbol = Symbol => ActionFn(13);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action13::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            25 => {
                (|| {
                    // TapeSymbol = "_" => ActionFn(14);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action14::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            26 => {
                (|| {
                    // TransArm = ShortTransArm => ActionFn(4);
                    let __sym0 = __pop_Variant11(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action4::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            27 => {
                (|| {
                    // TransArm = BlockTransArm => ActionFn(5);
                    let __sym0 = __pop_Variant5(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action5::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            28 => {
                (|| {
                    // Transition = State, TransArm => ActionFn(3);
                    let __sym1 = __pop_Variant5(__symbols);
                    let __sym0 = __pop_Variant3(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action3::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                    (2, __symbol, 18)
                })()
            }
            29 => {
                (|| {
                    // Transition+ = Transition => ActionFn(24);
                    let __sym0 = __pop_Variant14(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action24::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                    (1, __symbol, 19)
                })()
            }
            30 => {
                (|| {
                    // Transition+ = Transition+, Transition => ActionFn(25);
                    let __sym1 = __pop_Variant14(__symbols);
                    let __sym0 = __pop_Variant15(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action25::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                    (2, __symbol, 19)
                })()
            }
            31 => {
                (|| {
                    // Transitions = Transition+ => ActionFn(2);
                    let __sym0 = __pop_Variant15(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action2::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant16(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            32 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 22 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (State, Vec<TransitionArm>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Vec<State>, MultiMap<State, TransitionArm>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, InputAlphabet, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Move, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Pattern, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, State, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TapeSymbol, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TransitionArm, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Symbol>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<TransitionArm>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(State, Vec<TransitionArm>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Symbol>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<TransitionArm>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use ast::{Program, TransitionArm, Pattern};
    use dtm::{State, TapeSymbol, Symbol, Move, InputAlphabet};
    use utility::MultiMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:[0-9A-Za-z])+)",
                "^((?u:!))",
                "^((?u:\\*))",
                "^((?u:,))",
                "^((?u:\\->))",
                "^((?u::))",
                "^((?u:;))",
                "^((?u:<))",
                "^((?u:>))",
                "^((?u:_))",
                "^((?u:accept))",
                "^((?u:alphabet))",
                "^((?u:reject))",
                "^((?u:\\{))",
                "^((?u:\\}))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[0-9A-Za-z])+)").unwrap(),
                __regex::Regex::new("^((?u:!))").unwrap(),
                __regex::Regex::new("^((?u:\\*))").unwrap(),
                __regex::Regex::new("^((?u:,))").unwrap(),
                __regex::Regex::new("^((?u:\\->))").unwrap(),
                __regex::Regex::new("^((?u::))").unwrap(),
                __regex::Regex::new("^((?u:;))").unwrap(),
                __regex::Regex::new("^((?u:<))").unwrap(),
                __regex::Regex::new("^((?u:>))").unwrap(),
                __regex::Regex::new("^((?u:_))").unwrap(),
                __regex::Regex::new("^((?u:accept))").unwrap(),
                __regex::Regex::new("^((?u:alphabet))").unwrap(),
                __regex::Regex::new("^((?u:reject))").unwrap(),
                __regex::Regex::new("^((?u:\\{))").unwrap(),
                __regex::Regex::new("^((?u:\\}))").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 15 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, alphabet, _): (usize, InputAlphabet, usize),
    (_, accept, _): (usize, State, usize),
    (_, reject, _): (usize, State, usize),
    (_, transitions, _): (usize, (Vec<State>, MultiMap<State, TransitionArm>), usize),
) -> Program
{
    Program {
        alphabet,
        accept,
        reject,
        states: transitions.0,
        transitions: transitions.1
    }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<(State, Vec<TransitionArm>)>, usize),
) -> (Vec<State>, MultiMap<State, TransitionArm>)
{
    {
        let mut states = Vec::new();
        let mut map = MultiMap::new();
        for trans in __0 {
            states.push(trans.0.clone());
            map.insert_all(trans.0, trans.1);
        }
        (states, map)
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, from, _): (usize, State, usize),
    (_, arms, _): (usize, Vec<TransitionArm>, usize),
) -> (State, Vec<TransitionArm>)
{
    (from, arms)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TransitionArm, usize),
) -> Vec<TransitionArm>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<TransitionArm>, usize),
) -> Vec<TransitionArm>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<TransitionArm>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<TransitionArm>
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, read, _): (usize, Pattern, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, to, _): (usize, State, usize),
    (_, write, _): (usize, Pattern, usize),
    (_, mov, _): (usize, Move, usize),
) -> TransitionArm
{
    TransitionArm {
        read,
        to,
        write,
        mov
    }
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Symbol>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> InputAlphabet
{
    __0
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, State, usize),
    (_, _, _): (usize, &'input str, usize),
) -> State
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, State, usize),
    (_, _, _): (usize, &'input str, usize),
) -> State
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Pattern
{
    Pattern::Wildcard
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TapeSymbol, usize),
) -> Pattern
{
    Pattern::TapeSymbol(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
) -> TapeSymbol
{
    TapeSymbol::Symbol(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TapeSymbol
{
    TapeSymbol::Blanco
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Symbol
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_owned()
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Move
{
    Move::Left
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Move
{
    Move::None
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Move
{
    Move::Right
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, ::std::vec::Vec<Symbol>, usize),
    (_, b, _): (usize, Symbol, usize),
) -> Vec<Symbol>
{
    {
        let mut a = a;
        a.push(b);
        a
    }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TransitionArm, usize),
) -> ::std::vec::Vec<TransitionArm>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TransitionArm>, usize),
    (_, e, _): (usize, TransitionArm, usize),
) -> ::std::vec::Vec<TransitionArm>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (State, Vec<TransitionArm>), usize),
) -> ::std::vec::Vec<(State, Vec<TransitionArm>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(State, Vec<TransitionArm>)>, usize),
    (_, e, _): (usize, (State, Vec<TransitionArm>), usize),
) -> ::std::vec::Vec<(State, Vec<TransitionArm>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Symbol>
{
    vec![]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Symbol>, usize),
) -> ::std::vec::Vec<Symbol>
{
    v
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Symbol
{
    (__0)
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
) -> ::std::vec::Vec<Symbol>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Symbol>, usize),
    (_, e, _): (usize, Symbol, usize),
) -> ::std::vec::Vec<Symbol>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, Symbol, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Symbol>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Symbol>, usize),
    __1: (usize, Symbol, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Symbol>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action28(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, Symbol, usize),
) -> Vec<Symbol>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Symbol>, usize),
    __1: (usize, Symbol, usize),
) -> Vec<Symbol>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __temp0,
        __1,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
