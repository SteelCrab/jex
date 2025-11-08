use crate::lexer::Lexer;
use crate::lexer::*;

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_class_main(){
        let source_1="public class Main { }";
        let mut lexer = Lexer::new(source_1);
        let toknes = lexer.tokenize();

        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[0].token_type, TokenType::Public);
        assert_eq!(tokens[0].token_type, TokenType::Public);

    }
}