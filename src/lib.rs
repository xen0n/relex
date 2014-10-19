extern crate regex;


pub struct Token<'a> {
    typ: uint,
    frag: &'a str,
    span: (uint, uint),
    line: uint,
}

pub type Tokens<'a> = Vec<Token<'a>>;

pub struct TokenHandlerResult<'a> {
    skip: bool,
    tokens: Option<Tokens<'a>>,
}

pub type TokenHandler<'a> = |&'a str, &'a regex::Captures|: 'a -> &'a TokenHandlerResult;

pub struct LexRule<'a, 'r> {
    re: &'r regex::Regex,
    handler: TokenHandler<'a>,
}


pub fn lex<'a, 'r>(input: &'a str, rules: &[LexRule<'a, 'r>]) -> Option<Tokens<'a>> {
    None
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:
