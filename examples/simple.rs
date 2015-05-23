extern crate regex;

extern crate relex;

use regex::Captures;

use relex::token::Token;
use relex::lex::LexRule;
use relex::lex::Lexer;
use relex::lex::LexerResult;


#[cfg(not(test))]
fn main() {
    let test_input = "_haha { heheHEHE { x123; } } abcd".to_string();
    let mut lexer = Lexer::new(&test_input);

    lexer.add_rule(LexRule::new(
            r"^[A-Za-z_][0-9A-Za-z_-]*",
            Box::new(move |m: Captures| -> LexerResult {
                Some(vec![Token {
                    typ: 0,
                    frag: m.at(0).unwrap().to_string(),
                    span: m.pos(0).unwrap(),
                    line: 0usize,
                }])
            }),
            ));

    lexer.add_rule(LexRule::new(
            r"^\s+",
            Box::new(move |_m: Captures| -> LexerResult {
                Some(vec![])
            }),
            ));

    lexer.add_rule(LexRule::new(
            r"^\{",
            Box::new(move |m: Captures| -> LexerResult {
                Some(vec![Token {
                    typ: 1,
                    frag: m.at(0).unwrap().to_string(),
                    span: m.pos(0).unwrap(),
                    line: 0usize,
                }])
            }),
            ));

    lexer.add_rule(LexRule::new(
            r"^\}",
            Box::new(move |m: Captures| -> LexerResult {
                Some(vec![Token {
                    typ: 2,
                    frag: m.at(0).unwrap().to_string(),
                    span: m.pos(0).unwrap(),
                    line: 0usize,
                }])
            }),
            ));

    lexer.add_rule(LexRule::new(
            r"^;",
            Box::new(move |m: Captures| -> LexerResult {
                Some(vec![Token {
                    typ: 3,
                    frag: m.at(0).unwrap().to_string(),
                    span: m.pos(0).unwrap(),
                    line: 0usize,
                }])
            }),
            ));

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
