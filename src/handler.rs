extern crate regex;

use regex::Captures;

use super::LexerResult;


pub type RuleHandler<T, S> = Fn(Captures, &mut S) -> LexerResult<T>;

pub type PostProcessor<T> = Fn(LexerResult<T>) -> LexerResult<T>;
