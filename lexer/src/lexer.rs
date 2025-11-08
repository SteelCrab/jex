#[derive(Debug,Clone,PartialEq)]
pub enum TokenType{
    //Class & Access Modifiers && member
    Public, Class, Static,
    //Primitive Types
    Void, Int,
    //Var
    Identifier(String),
    //Number
    NumberInt(i32),
    //Operations
    OperationAssign,
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
#[derive(Debug,Clone)]
pub struct Token{
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
    /// Creates a new Lexer for the given source string.
    ///
    /// The lexer is initialized to start at the first character (position 0) and line 1.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("int x = 42;");
    /// let tokens = lexer.tokenize();
    /// assert!(!tokens.is_empty());
    /// ```
    pub fn new(source: &str) -> Self{
        Self{
            input: source.chars().collect(),
            position: 0,
            line: 1,
        }
    }
    
    /// Tokenizes the lexer's input into a sequence of lexical tokens.
    ///
    /// The returned vector contains all tokens produced from the current input
    /// position through end-of-file and always ends with a `Token` whose
    /// `token_type` is `TokenType::Eof`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("class A { }");
    /// let tokens = lexer.tokenize();
    /// assert!(!tokens.is_empty());
    /// assert!(matches!(tokens.last().unwrap().token_type, TokenType::Eof));
    /// ```
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
    
        while !self.is_at_end() {
            self.skip_whitespace();
            
            if self.is_at_end(){
                break;
            }

           if let Some(token) = self.next_token() {
                tokens.push(token);
           }
            
        }
        tokens.push(Token { token_type:TokenType::Eof, line: self.line });
       
        tokens
    }
    /// Checks whether the lexer has reached or passed the end of its input.
    ///
    /// Returns `true` if the current position is at or beyond the input length, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("");
    /// assert!(lexer.is_at_end());
    ///
    /// let mut lexer = Lexer::new("x");
    /// assert!(!lexer.is_at_end());
    /// ```
    fn is_at_end(&mut self) -> bool{
         self.position >= self.input.len()
    }
    /// Advances the lexer position past consecutive whitespace characters and increments the lexer's line counter for each newline encountered.
    ///
    /// Stops on the first non-whitespace character (or end of input).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = crate::lexer::Lexer::new(" \nclass");
    /// let tokens = lexer.tokenize();
    /// assert!(matches!(tokens.first().unwrap().token_type, crate::lexer::TokenType::Class));
    /// ```
    fn skip_whitespace(&mut self) {
        while let Some(ch) =  self.current_char() {
                if ch.is_whitespace() {
                    if ch =='\n'{
                        self.line += 1;
                    }
                    self.next_char();
                } else {
                    break;
                }
            }
    }

    /// Returns the character at the lexer's current position, if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut l = Lexer::new("ab");
    /// assert_eq!(l.current_char(), Some('a'));
    /// l.next_char();
    /// assert_eq!(l.current_char(), Some('b'));
    /// l.next_char();
    /// assert_eq!(l.current_char(), None);
    /// ```
    fn current_char(&self) -> Option<char>{
        self.input.get(self.position).copied()
    }
    /// Parses a single token starting at the lexer's current position.
    ///
    /// This method consumes characters needed to form the next token (punctuation, assignment,
    /// integer literals, or identifiers/keywords). Unknown single characters are skipped and parsing
    /// continues until a valid token is produced. The returned token carries the lexer's current
    /// line number captured at the start of parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("int x = 42;");
    /// let tokens = lexer.tokenize();
    /// assert!(matches!(tokens[0].token_type, TokenType::Int));
    /// assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
    /// assert!(matches!(tokens[3].token_type, TokenType::NumberInt(42)));
    /// ```
    fn next_token(&mut self) -> Option<Token> {
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
            ('a'..='z') | ('A'..= 'Z') => self.read_identifier(),

            _ => {
                self.next_char();
                return self.next_token();
            }
        };
        
        Some(Token { token_type, line })

    }
    /// Advance the lexer's current reading position by one character.
    ///
    /// This increments the internal position index so the lexer will consider the next input character.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = crate::lexer::Lexer::new("ab");
    /// // inside the same crate/module this private helper advances the cursor
    /// lexer.next_char();
    /// ```
    fn next_char(&mut self){
        self.position += 1;
    }
    
    /// Reads a consecutive sequence of decimal digits from the current position and returns it as a numeric token.
    ///
    /// The method consumes all contiguous numeric characters starting at the lexer's current position and produces `TokenType::NumberInt` containing the parsed `i32` (or `0` if parsing fails).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("42");
    /// let token = lexer.read_number();
    /// match token {
    ///     TokenType::NumberInt(n) => assert_eq!(n, 42),
    ///     _ => panic!("expected NumberInt"),
    /// }
    /// ```
    fn read_number(&mut self) -> TokenType {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_numeric(){
                num_str.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        let number_i32:i32 = num_str.parse().unwrap_or(0);
        TokenType::NumberInt(number_i32)

    }
    
    /// Reads an alphanumeric sequence at the lexer's current position and returns the corresponding token type.
    ///
    /// Recognizes the keywords `"class"`, `"public"`, `"static"`, `"void"`, and `"int"` and returns their respective `TokenType` variants; any other alphanumeric sequence is returned as `TokenType::Identifier(String)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = crate::lexer::Lexer::new("myVar");
    /// let tokens = lexer.tokenize();
    /// assert!(matches!(tokens[0].token_type, crate::lexer::TokenType::Identifier(ref s) if s == "myVar"));
    /// ```
    fn read_identifier(&mut self) -> TokenType {
        let mut word:String = String::new();
        
        while let Some(ch) = self.current_char(){
            if ch.is_alphanumeric() {
                word.push(ch);
                self.next_char();
            }else{
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
  