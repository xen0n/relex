// #![feature(phase)]

// #[phase(plugin, link)]
#[macro_use]
extern crate log;

extern crate regex;

use std::fmt;
//use std::rc::Rc;


enum TokenState {
    NothingToken,
    SomeToken(u32),
}

pub struct Token<'a> {
    pub typ: u32,
    pub frag: &'a str,
    pub span: (usize, usize),
    pub line: usize,
}

pub type Tokens<'a> = Vec<Token<'a>>;

pub enum TokenHandlerResult<'a> {
    SkipRule,
    Proceed(Tokens<'a>),
}

//pub type TokenHandler<'a> = |&'a Rc<regex::Captures>|: 'a -> TokenHandlerResult;
// pub type TokenHandler<'a, 'r> = |&'r regex::Captures|: 'a -> TokenHandlerResult<'a>;

pub struct LexRule<'a, 'r, TokenHandler>
where TokenHandler: Fn(&'r regex::Captures) -> TokenHandlerResult<'a> {
    pub re: regex::Regex,
    // pub handler: TokenHandler<'a, 'r>,
    pub handler: TokenHandler,
}

struct LexerState {
    pos: usize,
    prev: TokenState,
}


impl<'a, 'r, TokenHandler> fmt::Show for LexRule<'a, 'r, TokenHandler>
where TokenHandler: Fn(&'r regex::Captures) -> TokenHandlerResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexRule {{ re: regex!(r\"{}\") }}", self.re)
    }
}

impl fmt::Show for LexerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerState(pos={})", self.pos)
    }
}


pub fn lex<'a, 'r, TokenHandler>(input: &'a String, rules: &'a mut [LexRule<'a, 'r, TokenHandler>]) -> Option<Tokens<'a>>
where TokenHandler: Fn(&'r regex::Captures) -> TokenHandlerResult<'a> {
    let mut state = LexerState {
        pos: 0us,
        prev: TokenState::NothingToken,
    };
    let mut result = Vec::new();

    let last_pos = input.len() - 1us;

    debug!("lex: input={}, last_pos={}", input, last_pos);

    while state.pos < last_pos {
        match lex_advance(input, rules, &mut state) {
            Some(tokens) => {
                debug!("lex: ADVANCE: curr_len={}, new_len={}", result.len(), tokens.len());
                result.extend(tokens.into_iter());
            },
            None => {
                return None;
            }
        }
    }

    Some(result)
}


fn lex_advance<'a, 'r, 's, TokenHandler>(input: &'a String, rules: &'a mut [LexRule<'a, 'r, TokenHandler>], state: &'s mut LexerState) -> Option<Tokens<'a>>
where TokenHandler: Fn(&'r regex::Captures) -> TokenHandlerResult<'a> {
    let input_ahead = input[&state.pos .. ];

    // ???
    let mut capture_groups = Vec::new();

    debug!("advance: ahead={}, state.pos={}", input_ahead, state.pos);
    for rule in rules.iter_mut() {
        debug!("advance: TRY: {}", rule);
        match rule.re.find(input_ahead) {
            Some((span_start, span_end)) => {
                debug!("advance: YES: {}, span=({}, {})", rule, span_start, span_end);
                // let captures = box rule.re.captures(input_ahead).unwrap();
                capture_groups.push(rule.re.captures(input_ahead).unwrap());

                match ((*rule).handler)(&capture_groups[capture_groups.len()]) {
                    TokenHandlerResult::SkipRule => {
                        continue;
                    },
                    TokenHandlerResult::Proceed(tokens) => {
                        state.pos += span_end;
                        if tokens.len() > 0 {
                            state.prev = TokenState::SomeToken(tokens[tokens.len() - 1].typ);
                        }

                        return Some(tokens);
                    },
                }
            },
            None => {
                debug!("advance: NO: {}", rule);
                continue;
            }
        }
    }

    None
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:
