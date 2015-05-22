#![feature(phase)]

extern crate regex;

#[phase(plugin)]
extern crate regex_macros;

extern crate relex;

use std::rc::Rc;


#[cfg(not(test))]
fn main() {
    let test_input = String::from_str("_haha { heheHEHE { x123; } } abcd");
    let mut rules = [
        relex::LexRule {
            re: regex!(r"^[A-Za-z_][0-9A-Za-z_-]*"),
            handler: |m: &Rc<regex::Captures>| -> relex::TokenHandlerResult {
                let frag = m.clone().at(0);
                let span = m.clone().pos(0).unwrap();

                relex::Proceed(vec![relex::Token {
                    typ: 0,
                    frag: frag,
                    span: span,
                    line: 0u,
                }])
            },
        },
        relex::LexRule {
            re: regex!(r"^\s+"),
            handler: |m: &Rc<regex::Captures>| -> relex::TokenHandlerResult {
                relex::Proceed(vec![])
            },
        },
        relex::LexRule {
            re: regex!(r"^\{"),
            handler: |m: &Rc<regex::Captures>| -> relex::TokenHandlerResult {
                relex::Proceed(vec![relex::Token {
                    typ: 1,
                    frag: (*m).at(0),
                    span: (*m).pos(0).unwrap(),
                    line: 0u,
                }])
            },
        },
        relex::LexRule {
            re: regex!(r"^\}"),
            handler: |m: &Rc<regex::Captures>| -> relex::TokenHandlerResult {
                relex::Proceed(vec![relex::Token {
                    typ: 2,
                    frag: (*m).at(0),
                    span: (*m).pos(0).unwrap(),
                    line: 0u,
                }])
            },
        },
        relex::LexRule {
            re: regex!(r"^;"),
            handler: |m: &Rc<regex::Captures>| -> relex::TokenHandlerResult {
                relex::Proceed(vec![relex::Token {
                    typ: 3,
                    frag: (*m).at(0),
                    span: (*m).pos(0).unwrap(),
                    line: 0u,
                }])
            },
        },
    ];

    let parse_result = relex::lex(&test_input, &mut rules);

    match parse_result {
        Some(thing) => (),
        None => {
            println!("Parse failed!");
        }
    };
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:
