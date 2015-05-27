extern crate regex;

#[macro_use]
extern crate relex;

use regex::Captures;

use relex::token::Token;
use relex::token::Span;
use relex::lex::Lexer;
use relex::LexerResult;

type U32Token = Token<u32>;
type U32Lexer<'a> = Lexer<'a, U32Token, u32>;
type U32LexerResult = LexerResult<U32Token>;


fn main() {
    let test_input = "_haha { heheHEHE { x123; } } abcd".to_string();

    // state is not used in this example
    let mut lexer = U32Lexer::new(&test_input, 0);

    add_lex_rule!(
            lexer,
            r"^[A-Za-z_][0-9A-Za-z_-]*",
            move |m: Captures, sp: Span, _eof: bool, _inp: &String, _s: &mut u32| -> U32LexerResult {
                Some(vec![U32Token {
                    typ: 0,
                    frag: m.at(0).unwrap().to_string(),
                    span: sp,
                    line: 0usize,
                }])
            },
            );

    add_lex_rule!(
            lexer,
            r"^\s+",
            move |_m: Captures, _sp: Span, _eof: bool, _inp: &String, _s: &mut u32| -> U32LexerResult {
                Some(vec![])
            },
            );

    add_lex_rule!(
            lexer,
            r"^\{",
            move |m: Captures, sp: Span, _eof: bool, _inp: &String, _s: &mut u32| -> U32LexerResult {
                Some(vec![U32Token {
                    typ: 1,
                    frag: m.at(0).unwrap().to_string(),
                    span: sp,
                    line: 0usize,
                }])
            },
            );

    add_lex_rule!(
            lexer,
            r"^\}",
            move |m: Captures, sp: Span, _eof: bool, _inp: &String, _s: &mut u32| -> U32LexerResult {
                Some(vec![U32Token {
                    typ: 2,
                    frag: m.at(0).unwrap().to_string(),
                    span: sp,
                    line: 0usize,
                }])
            },
            );

    add_lex_rule!(
            lexer,
            r"^;",
            move |m: Captures, sp: Span, _eof: bool, _inp: &String, _s: &mut u32| -> U32LexerResult {
                Some(vec![U32Token {
                    typ: 3,
                    frag: m.at(0).unwrap().to_string(),
                    span: sp,
                    line: 0usize,
                }])
            },
            );

    println!("Parsing input: {:?}", test_input);
    let parse_result = lexer.lex();

    match parse_result {
        Some(thing) => {
            println!("Parse finished: {:?}", thing);
        },
        None => {
            println!("Parse failed!");
        }
    };
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:
