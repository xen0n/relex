extern crate regex;

#[macro_use]
extern crate relex;

use regex::Captures;

use relex::token::Span;
use relex::lex::Lexer;
use relex::Wstr;
use relex::LexerResult;

#[derive(Debug, Clone)]
enum WRLTokenType {
    NEWLINE,
    INDENT,
    DEDENT,
    ATTRIB(String),
    COLON,
    LITERAL(String),
}

#[derive(Debug)]
struct WRLToken {
    typ: WRLTokenType,
    span: (usize, usize),
    line: usize,
}

#[derive(Debug)]
struct WRLLexerState {
    lineno: usize,
    indstk: Vec<usize>,
    is_prev_space_empty: bool,
}

type WRLLexer<'a> = Lexer<'a, WRLToken, WRLLexerState>;
type WRLLexerResult = LexerResult<WRLToken>;


impl Clone for WRLToken {
    fn clone(&self) -> Self {
        WRLToken {
            typ: self.typ.clone(),
            span: self.span,
            line: self.line,
        }
    }
}


fn main() {
    let test_input = "\
--exact
--default-type=regex
--renderer=json

# Static file root.
/static/:
    # dummy
    ^(?P<path>.*)$ staticfile null

# Multi-formatted view demo.
# dummy
# only for testing
# the lexer implementation
/calc/:
    ^(?P<val>\\d+)/:
        json/$ multiformat-test  # This uses the default renderer.
        text/$ multiformat-test mako

# fallback
\"\" index
".to_string();

    let mut lexer = WRLLexer::new(&test_input, WRLLexerState {
        lineno: 1,
        indstk: { let mut vec = Vec::with_capacity(8); vec.push(0); vec },
        is_prev_space_empty: false,
    });

    // LINECOMMENT
    add_lex_rule!(
            lexer,
            r"^[ \t]*#[^\n]*",
            move |_m: Captures, _sp: Span, _eof: bool, _inp: &Wstr, _s: &mut WRLLexerState| -> WRLLexerResult {
                Some(vec![])
            },
            );

    // ATTRIB
    add_lex_rule!(
            lexer,
            r"^--([^\s]+)",
            move |m: Captures, sp: Span, _eof: bool, _inp: &Wstr, s: &mut WRLLexerState| -> WRLLexerResult {
                Some(vec![WRLToken {
                    typ: WRLTokenType::ATTRIB(m.at(1).unwrap().to_string()),
                    span: sp,
                    line: s.lineno,
                }])
            },
            );

    // NEWLINE
    add_lex_rule!(
            lexer,
            r"^\n+",
            move |m: Captures, sp: Span, _eof: bool, _inp: &Wstr, s: &mut WRLLexerState| -> WRLLexerResult {
                let result = vec![WRLToken {
                    typ: WRLTokenType::NEWLINE,
                    span: sp,
                    line: s.lineno,
                }];

                // advance lineno after (effectively) emitting the token for
                // saner lineno reporting
                // span always start at index 0 so we can optimize a bit
                let (_span_start, span_end) = m.pos(0).unwrap();
                s.lineno += span_end;

                Some(result)
            },
            );

    // SPACE and EOF
    add_lex_rule!(
            lexer,
            r"^[ \t]*|^$",
            move |m: Captures, sp: Span, eof: bool, inp: &Wstr, s: &mut WRLLexerState| -> WRLLexerResult {
                if s.is_prev_space_empty {
                    s.is_prev_space_empty = false;
                    return None;
                }

                let (sp_start, sp_end) = sp;
                let match_length = sp_end - sp_start;

                if match_length == 0 {
                    s.is_prev_space_empty = true;
                }

                // we're only interested in leading whitespaces and EOF
                // consume non-leading whitespaces and emit nothing
                if !eof && inp[sp_start - 1] != '\n' {
                    return Some(vec![]);
                }

                // process indentation
                // calculate effective width
                let frag = m.at(0).unwrap();
                let mut efflen = 0usize;
                for ch in frag.chars() {
                    efflen += match ch {
                        '\t' => 8 - efflen % 8,
                        ' ' => 1,
                        _ => panic!("non-whitespace char in SPACE \"token\""),
                    }
                }

                let stktop = *(s.indstk.last().unwrap());

                // we're at the same indentation level?
                // return early if that's the case
                if stktop == efflen {
                    return Some(vec![]);
                }

                // INDENT handling
                if stktop < efflen {
                    s.indstk.push(efflen);
                    return Some(vec![WRLToken {
                        typ: WRLTokenType::INDENT,
                        span: sp,
                        line: s.lineno,
                    }]);
                }

                // DEDENT check
                let mut dedents = Vec::with_capacity(8);
                let mut stktop = stktop;
                while stktop > efflen {
                    dedents.push(WRLToken {
                        typ: WRLTokenType::DEDENT,
                        span: sp,
                        line: s.lineno,
                    });
                    s.indstk.pop();
                    stktop = *(s.indstk.last().unwrap());
                }

                // bark if indentation isn't consistent
                if stktop != efflen {
                    // TODO: properly raise parse error
                    panic!("inconsistent indentation");
                }

                // return the emitted DEDENTs
                Some(dedents)
            },
            );

    // COLON
    add_lex_rule!(
            lexer,
            r"^:\n",
            move |_m: Captures, sp: Span, _eof: bool, _inp: &Wstr, s: &mut WRLLexerState| -> WRLLexerResult {
                let result = vec![WRLToken {
                    typ: WRLTokenType::COLON,
                    span: sp,
                    line: s.lineno,
                }];

                // advance lineno because there is no NEWLINE for colons
                s.lineno += 1;

                Some(result)
            },
            );

    // LITERAL
    add_lex_rule!(
            lexer,
            r##"^[^#'"\s]*[^#'"\s:]|'[^']*'|"[^"]*""##,
            move |m: Captures, sp: Span, _eof: bool, _inp: &Wstr, s: &mut WRLLexerState| -> WRLLexerResult {
                let frag = m.at(0).unwrap();
                let actual_frag = match frag.chars().next().unwrap() {
                    '\'' | '"' => unsafe { frag.slice_unchecked(1, frag.len() - 1) },
                    _ => frag,
                };

                Some(vec![WRLToken {
                    typ: WRLTokenType::LITERAL(actual_frag.to_string()),
                    span: sp,
                    line: s.lineno,
                }])
            },
            );

    // Postprocessor for token stream.
    lexer.set_postprocessor(Box::new(move |before: WRLLexerResult| -> WRLLexerResult {
        let tokens = before.unwrap();

        // Coalesce consecutive NEWLINEs generated by multiple lines of
        // line comments.
        let mut result = Vec::with_capacity(tokens.len());
        let mut last_tok_type_is_nl = false;
        let mut last_out_tok_type_is_colon = false;
        let mut have_merged_tok = false;
        let mut merged_tok_span_start = 0usize;
        let mut merged_tok_span_end = 0usize;
        let mut merged_tok_lineno = 0usize;

        for tok in tokens.iter() {
            if let WRLTokenType::NEWLINE = tok.typ {
            } else {
                // Pass-through.
                // But first see if we have any pending merged NEWLINEs.
                if have_merged_tok {
                    // Don't output NEWLINE immediately after COLON
                    if !last_out_tok_type_is_colon {
                        // construct the merged token once
                        let merged_out_tok = WRLToken {
                            typ: WRLTokenType::NEWLINE,
                            span: (merged_tok_span_start, merged_tok_span_end),
                            line: merged_tok_lineno,
                        };

                        result.push(merged_out_tok);
                    }

                    have_merged_tok = false;
                }

                // Pass-through all the non-NEWLINE tokens.
                result.push(tok.clone());
                last_out_tok_type_is_colon = if let WRLTokenType::COLON = tok.typ {
                    true
                } else {
                    false
                };
                last_tok_type_is_nl = if let WRLTokenType::NEWLINE = tok.typ {
                    true
                } else {
                    false
                };

                continue;
            }

            // Is this NEWLINE the first in a row?
            let (ss, se) = tok.span;

            if !last_tok_type_is_nl {
                // Yes, record its location info and suppress it
                have_merged_tok = true;
                merged_tok_span_start = ss;
                merged_tok_lineno = tok.line;
            }

            // Now in a NEWLINE run, suppress and merge the consecutive spans
            merged_tok_span_end = se;
        }

        Some(result)
    }));

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
