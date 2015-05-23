use super::LexerResult;
use super::token::TokenState;
use super::token::NextTokenState;
use super::rule::LexRule;
use super::rule::LexRuleMatch;


pub struct Lexer<'a> {
    source: &'a String,
    rules: Vec<LexRule>,
    // pos: usize,
    // prev: TokenState,
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source: source,
            rules: Vec::new(),
            // pos: 0usize,
            // prev: None,
        }
    }

    /*
    fn reset(&mut self) {
        self.pos = 0usize;
        self.prev = None;
    }
    */

    pub fn add_rule(&mut self, rule: LexRule) {
        self.rules.push(rule);
    }

    pub fn lex(&mut self) -> LexerResult {
        // self.reset();

        let last_pos = self.source.len() - 1usize;
        debug!("lex: input={}, last_pos={}", self.source, last_pos);

        let mut pos = 0usize;
        let mut _prev : TokenState = None;
        let mut result = Vec::new();
        while pos < last_pos {
            // let curr_pos = pos;
            let next_state = self.advance(pos);

            if let Some(tokens) = next_state.result {
                // stash result
                result.extend(tokens.into_iter());

                // update state
                if let NextTokenState::NewToken(next_token_state) = next_state.new_state {
                    _prev = Some(next_token_state);
                }

                pos += next_state.advance;
                // TODO: lineno
            } else {
                return None;
            }
        }

        Some(result)
    }

    fn advance(&self, pos: usize) -> LexRuleMatch {
        // slice the input
        let input_ahead = &self.source[pos ..];

        for rule in self.rules.iter() {
            let rule_match = rule.execute(input_ahead);
            if let Some(_) = rule_match.result {
                return rule_match;
            }
        }

        LexRuleMatch {
            result: None,
            new_state: NextTokenState::Keep,
            advance: 0,
            advance_lineno: 0,
        }
    }
}
