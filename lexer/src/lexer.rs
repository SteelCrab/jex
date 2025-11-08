#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //Class & Access Modifiers && member
    Public,
    Class,
    Static,
    //Primitive Types
    Void,
    Int,
    //Var
    Identifier(String),
    //Number
    NumberInt(i32),
    //Operations
    OperationAssign,
    // { }
    LeftBrace,
    RightBrace,
    // ( )
    LeftParen,
    RightParen,
    // ;
    Semicolon,
    // Done
    Eof,
}
// Token
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}
//lexer
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
        });

        tokens
    }
    fn is_at_end(&mut self) -> bool {
        self.position >= self.input.len()
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                }
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    fn next_token(&mut self) -> Option<Token> {
        let line = self.line;
        let ch = self.current_char().unwrap();
        let token_type = match ch {
            '{' => {
                self.next_char();
                TokenType::LeftBrace
            }
            '}' => {
                self.next_char();
                TokenType::RightBrace
            }
            '(' => {
                self.next_char();
                TokenType::LeftParen
            }
            ')' => {
                self.next_char();
                TokenType::RightParen
            }
            ';' => {
                self.next_char();
                TokenType::Semicolon
            }
            '=' => {
                self.next_char();
                TokenType::OperationAssign
            }
            //read num
            '0'..='9' => self.read_number(),
            //read identifier
            ('a'..='z') | ('A'..='Z') => self.read_identifier(),

            _ => {
                self.next_char();
                return self.next_token();
            }
        };

        Some(Token { token_type, line })
    }
    fn next_char(&mut self) {
        self.position += 1;
    }

    fn read_number(&mut self) -> TokenType {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_numeric() {
                num_str.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        let number_i32: i32 = num_str.parse().unwrap_or(0);
        TokenType::NumberInt(number_i32)
    }

    fn read_identifier(&mut self) -> TokenType {
        let mut word: String = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() {
                word.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        match word.as_str() {
            "class" => TokenType::Class,
            "public" => TokenType::Public,
            "static" => TokenType::Static,
            "void" => TokenType::Void,
            "int" => TokenType::Int,
            _ => TokenType::Identifier(word),
        }
    }
}
