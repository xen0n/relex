#[macro_use]
extern crate log;

extern crate regex;

pub mod token;
pub mod rule;
pub mod handler;
pub mod lex;

pub type LexerResult<T> = Option<Vec<T>>;


#[macro_export]
macro_rules! add_lex_rule {
    ($lexer:ident, $expr:expr, $handler:expr, ) => {
        $lexer.add_rule(relex::rule::LexRule::new($expr, Box::new($handler)));
    }
}
