
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
