extern crate regex;

use regex::Captures;

use super::LexerResult;


pub type RuleHandler<T> = Fn(Captures) -> LexerResult<T>;

pub type PostProcessor<T> = Fn(LexerResult<T>) -> LexerResult<T>;
