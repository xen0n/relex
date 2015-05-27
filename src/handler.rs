extern crate regex;

use regex::Captures;

use super::LexerResult;
use super::token::Span;


pub type RuleHandler<T, S> = Fn(Captures, Span, &mut S) -> LexerResult<T>;

pub type PostProcessor<T> = Fn(LexerResult<T>) -> LexerResult<T>;
