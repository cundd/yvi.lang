#[derive(Debug,PartialEq)]
pub struct Error {}

impl Token {
    pub fn token_for_char(input: char) -> Option<Token> {
        match input {
            '\n' => Some(Token::Eof),

            '{' => Some(Token::BlockStart),
            '}' => Some(Token::BlockEnd),

            '(' => Some(Token::TypeStart),
            ')' => Some(Token::TypeEnd),

            '[' => Some(Token::MemoryStart),
            ']' => Some(Token::MemoryEnd),

            '=' => Some(Token::Assignment),

            '<' => Some(Token::Return),

            '#' => Some(Token::Comment),

            ':' => Some(Token::PropertyAssignment),

            _ => None,
        }
    }

    fn analyze_string(input: &str) -> Result<Token> {


        // String(String), // "[^"]*"
        // Float(f64), // [0-9.]+
        // Int(i64), // [0-9]+
        Token::Identifier(input.to_string())
    }
    pub fn token_for_string(input: &str) -> Token {
        if input.len() == 1 {
            match Token::token_for_char(input.chars().nth(0).unwrap()) {
                Some(token) => return token,
                None => (),
            };
        }
        match input {
            "if" => Token::If,
            "else" => Token::Else,
            "bin" => Token::Bin,
            "new" => Token::New,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::analyze_string(input),
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_for_char() {
        assert_eq!(Token::BlockStart, Token::token_for_char('{').unwrap());
        assert_eq!(Token::BlockEnd, Token::token_for_char('}').unwrap());

        assert_eq!(Token::TypeStart, Token::token_for_char('(').unwrap());
        assert_eq!(Token::TypeEnd, Token::token_for_char(')').unwrap());

        assert_eq!(Token::Eof, Token::token_for_char('\n').unwrap());

        assert_eq!(Token::MemoryStart, Token::token_for_char('[').unwrap());
        assert_eq!(Token::MemoryEnd, Token::token_for_char(']').unwrap());

        assert_eq!(Token::Assignment, Token::token_for_char('=').unwrap());

        assert_eq!(Token::Return, Token::token_for_char('<').unwrap());

        assert_eq!(Token::Comment, Token::token_for_char('#').unwrap());

        assert_eq!(Token::PropertyAssignment,
                   Token::token_for_char(':').unwrap());

        assert_eq!(None, Token::token_for_char('a'));
        assert_eq!(None, Token::token_for_char('-'));
        assert_eq!(None, Token::token_for_char('&'));
    }

    fn assert_identifier_token_with_content(token: Token, content: &str) {
        match token {
            Token::Identifier(s) => {
                if s != content {
                    panic!("Identifier token value must be '{}'", content)
                }
            }
            _ => panic!("Token must be Token::Identifier"),
        }
    }

    #[test]
    fn token_for_string() {
        assert_eq!(Token::If, Token::token_for_string("if"));
        assert_eq!(Token::Else, Token::token_for_string("else"));

        assert_eq!(Token::Bin, Token::token_for_string("bin"));
        assert_eq!(Token::New, Token::token_for_string("new"));

        assert_eq!(Token::True, Token::token_for_string("true"));
        assert_eq!(Token::False, Token::token_for_string("false"));

        assert_identifier_token_with_content(Token::token_for_string("anything-else"),
                                             "anything-else");

        assert_identifier_token_with_content(Token::token_for_string(" "), " ");
        assert_identifier_token_with_content(Token::token_for_string("\t"), "\t");

        assert_eq!(Token::BlockStart, Token::token_for_string("{"));
        assert_eq!(Token::BlockEnd, Token::token_for_string("}"));

        assert_eq!(Token::TypeStart, Token::token_for_string("("));
        assert_eq!(Token::TypeEnd, Token::token_for_string(")"));

        assert_eq!(Token::Eof, Token::token_for_string("\n"));

        assert_eq!(Token::MemoryStart, Token::token_for_string("["));
        assert_eq!(Token::MemoryEnd, Token::token_for_string("]"));

        assert_eq!(Token::Assignment, Token::token_for_string("="));
    }
}
