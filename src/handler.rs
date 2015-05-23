extern crate regex;

use regex::Captures;

use super::LexerResult;


pub type RuleHandler = Fn(Captures) -> LexerResult;
