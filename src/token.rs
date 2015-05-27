#[derive(Debug)]
pub struct Token<T> {
    pub typ: T,
    pub frag: String,
    pub span: (usize, usize),
    pub line: usize,
}

pub type Span = (usize, usize);

/*
pub type TokenState<T> = Option<TokenType<T>>;


pub enum NextTokenState<T> {
    Keep,
    NewToken(TokenType<T>),
}
*/
