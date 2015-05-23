extern crate regex;

use regex::Captures;

use super::LexerResult;


pub type RuleHandler<T> = Fn(Captures) -> LexerResult<T>;
