pub type TokenType = u32;


#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub frag: String,
    pub span: (usize, usize),
    pub line: usize,
}


pub type TokenState = Option<TokenType>;


pub enum NextTokenState {
    Keep,
    NewToken(TokenType),
}
