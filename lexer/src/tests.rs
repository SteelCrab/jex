use crate::lexer::Lexer;
use crate::lexer::TokenType;

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_class_main(){
        let source_1="public class Main { }";
        let mut lexer = Lexer::new(source_1);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[1].token_type, TokenType::Class);
        assert_eq!(tokens[2].token_type, TokenType::Identifier("Main".to_owned()));
        assert_eq!(tokens[3].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[4].token_type, TokenType::RightBrace);
    }
}