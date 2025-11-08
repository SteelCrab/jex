
pub enum TokenType{
    //keyword
    Class, Public, Static, Void,
    //Var
    Identifier(String),
    //Number
    Number(i32),
    // { }
    LeftBrace, RightBrace,
    // ( )
    LeftParen, RightParen,
    // ;
    Semicolon,                       
    // Done
    Eof,

}
// Token
pub struct Token{
    pub token_type: TokenType,
    pub line: usize,
}