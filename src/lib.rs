#[macro_use]
extern crate log;

extern crate regex;

pub mod token;
pub mod rule;
pub mod handler;
pub mod lex;

pub type LexerResult = Option<Vec<self::token::Token>>;
