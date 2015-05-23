extern crate regex;

use regex::Regex;

use super::LexerResult;
use super::token::NextTokenState;
use super::handler::RuleHandler;


pub struct LexRule {
    rule: Regex,
    handler: Box<RuleHandler>,
}


pub struct LexRuleMatch {
    pub result: LexerResult,
    pub new_state: NextTokenState,
    pub advance: usize,
    pub advance_lineno: usize,
}


impl LexRule {
    pub fn new(rule: &str, handler: Box<RuleHandler>) -> LexRule {
        LexRule {
            rule: Regex::new(rule).unwrap(),
            handler: handler,
        }
    }

    pub fn execute(&self, input: &str) -> LexRuleMatch {
        if let Some(captures) = self.rule.captures(input) {
            let (_span_start, span_end) = captures.pos(0).unwrap();
            debug!("LexRule::execute: YES: {}, span_end={}", self.rule, span_end);

            if let Some(tokens) = (self.handler)(captures) {
                let new_token_state = if tokens.len() == 0 {
                    NextTokenState::Keep
                } else {
                    NextTokenState::NewToken(tokens[tokens.len() - 1].typ)
                };

                LexRuleMatch {
                    result: Some(tokens),
                    new_state: new_token_state,
                    advance: span_end,
                    advance_lineno: 0,
                }
            } else {
                LexRuleMatch {
                    result: None,
                    new_state: NextTokenState::Keep,
                    advance: 0,
                    advance_lineno: 0,
                }
            }
        } else {
            LexRuleMatch {
                result: None,
                new_state: NextTokenState::Keep,
                advance: 0,
                advance_lineno: 0,
            }
        }
    }
}
