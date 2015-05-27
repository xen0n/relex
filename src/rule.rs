extern crate regex;

use regex::Regex;

use super::LexerResult;
use super::handler::RuleHandler;


pub struct LexRule<T, S> {
    rule: Regex,
    handler: Box<RuleHandler<T, S>>,
}


pub struct LexRuleMatch<T> {
    pub result: LexerResult<T>,
    pub advance: usize,
}


impl<T, S> LexRule<T, S> {
    pub fn new(rule: &str, handler: Box<RuleHandler<T, S>>) -> LexRule<T, S> {
        LexRule {
            rule: Regex::new(rule).unwrap(),
            handler: handler,
        }
    }

    pub fn execute(&self, input: &str, state: &mut S) -> LexRuleMatch<T> {
        if let Some(captures) = self.rule.captures(input) {
            let (_span_start, span_end) = captures.pos(0).unwrap();
            debug!("LexRule::execute: YES: {}, span_end={}", self.rule, span_end);

            if let Some(tokens) = (self.handler)(captures, state) {
                LexRuleMatch {
                    result: Some(tokens),
                    advance: span_end,
                }
            } else {
                LexRuleMatch {
                    result: None,
                    advance: 0,
                }
            }
        } else {
            LexRuleMatch {
                result: None,
                advance: 0,
            }
        }
    }
}
