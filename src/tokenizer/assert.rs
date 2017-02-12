use super::token::Token;
use super::token::Token::Identifier;
use super::token::Token::Text;

#[allow(dead_code)]
pub struct Assert;

#[allow(dead_code)]
impl Assert {
    pub fn identifier_token(token: &Token, content: &str) {
        match token {
            &Identifier(ref s) => {
                assert_eq!(content, s, "Token value must be '{}' got '{}'", content, s)
            }
            _ => panic!("Token must be Token::Identifier got {:?}", token),
        }
    }

    pub fn text_token(token: &Token, content: &str) {
        match token {
            &Text(ref s) => assert_eq!(content, s, "Token value must be '{}' got '{}'", content, s),
            _ => panic!("Token must be Token::Text got {:?}", token),
        }
    }

    pub fn int_token(token: &Token, amount: i64) {
        match token {
            &Token::Int(s) => assert_eq!(amount, s, "Token value must be '{}' got '{}'", amount, s),
            _ => panic!("Token must be Token::Int got {:?}", token),
        }
    }

    pub fn float_token(token: &Token, amount: f64) {
        println!("{:?}", token);
        match token {
            &Token::Float(s) => {
                assert_eq!(amount, s, "Token value must be '{}' got '{}'", amount, s)
            }
            _ => panic!("Token must be Token::Float got {:?}", token),
        }
    }

    pub fn assert_complex_token(expected: &str, actual: &Token) {
        assert_eq!(expected, format!("{:?}", actual));
    }
}
