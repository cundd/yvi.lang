use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Single character tokens:
    // \n
    Eof,
    // #
    Comment,

    // (
    TypeStart,
    // )
    TypeEnd,

    // {
    BlockStart,
    // }
    BlockEnd,

    // [
    MemoryStart,
    // ]
    MemoryEnd,

    // =
    Assignment,

    // :
    PropertyAssignment,

    // <
    Return,

    // +
    MathAddition,
    // -
    MathSubtraction,
    // *
    MathMultiplication,
    // ÷ /
    MathDivision,

    // Keywords:
    // if
    If,
    // else
    Else,
    // bin
    Bin,
    // new
    New,
    // and
    And,
    // or
    Or,

    // Builtin types
    // "[^"]*"
    Text(String),
    // [0-9.]+
    Float(f64),
    // [0-9]+
    Int(i64),
    // Not a number
    NaN,

    // true
    True,
    // false
    False,

    // Identifier:
    // anything else
    Identifier(String),
}


// impl PartialEq for Token {
//     fn eq(&self, other: &Token) -> bool {
//         self == other
//     }
// }
//

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

            '+' => Some(Token::MathAddition),
            '-' => Some(Token::MathSubtraction),
            '*' => Some(Token::MathMultiplication),
            '/' => Some(Token::MathDivision),
            '÷' => Some(Token::MathDivision),

            '#' => Some(Token::Comment),

            ':' => Some(Token::PropertyAssignment),

            _ => None,
        }
    }

    fn analyze_string(input: &str) -> Result<Token, &'static str> {
        // String(String), // "[^"]*"
        // Float(f64), // [0-9.]+
        // Int(i64), // [0-9]+
        Ok(Token::Identifier(input.to_string()))
    }

    pub fn token_for_string(input: &str) -> Result<Token, &'static str> {
        if input.len() == 1 {
            match Token::token_for_char(input.chars().nth(0).unwrap()) {
                Some(token) => return Ok(token),
                None => (),
            };
        }
        match input {
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "bin" => Ok(Token::Bin),
            "new" => Ok(Token::New),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            _ => Token::analyze_string(input),
        }
    }

    pub fn token_for_number(input: &str) -> Token {
        if input == "NaN" {
            return Token::NaN;
        }

        if let Ok(value) = i64::from_str(input) {
            println!("value i64: {:?}", value);
            return Token::Int(value);
        }
        if let Ok(value) = f64::from_str(input) {
            println!("value f64: {:?}", value);
            return Token::Float(value);
        }

        Token::NaN
        //        Err("Could not convert number")
        //        Ok(Token::Int(2))
        // Ok(Token::Float(value))
        //         panic!("Could not convert number")


        // if let f = i64::from_str(input) {}
        //     Ok(int_value) => Ok(Token::Int(int_value)),
        //     Err => {
        //         match f64::from_str(input) {
        //             Ok(int_value) => Ok(Token::Float(int_value)),
        //             Err(e) => panic!(e),
        //         }
        //     }
        // }

        //::from_str(input);
        // input.to_string().parse::<i64>())
    }
}


#[cfg(test)]
mod tests {
    use std;
    use super::*;
    use super::super::assert::Assert;

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

        assert_eq!(Token::MathSubtraction, Token::token_for_char('-').unwrap());
        assert_eq!(Token::MathAddition, Token::token_for_char('+').unwrap());
        assert_eq!(Token::MathMultiplication,
        Token::token_for_char('*').unwrap());
        assert_eq!(Token::MathDivision, Token::token_for_char('/').unwrap());
        assert_eq!(Token::MathDivision, Token::token_for_char('÷').unwrap());

        assert_eq!(None, Token::token_for_char('a'));
        assert_eq!(None, Token::token_for_char('&'));
    }

    #[test]
    fn token_for_string() {
        assert_eq!(Token::If, Token::token_for_string("if").unwrap());
        assert_eq!(Token::Else, Token::token_for_string("else").unwrap());

        assert_eq!(Token::Bin, Token::token_for_string("bin").unwrap());
        assert_eq!(Token::New, Token::token_for_string("new").unwrap());

        assert_eq!(Token::True, Token::token_for_string("true").unwrap());
        assert_eq!(Token::False, Token::token_for_string("false").unwrap());

        assert_eq!(Token::And, Token::token_for_string("and").unwrap());
        assert_eq!(Token::Or, Token::token_for_string("or").unwrap());

        Assert::identifier_token(&Token::token_for_string("anything-else").unwrap(),
                                 "anything-else");

        Assert::identifier_token(&Token::token_for_string(" ").unwrap(), " ");
        Assert::identifier_token(&Token::token_for_string("\t").unwrap(), "\t");

        assert_eq!(Token::BlockStart, Token::token_for_string("{").unwrap());
        assert_eq!(Token::BlockEnd, Token::token_for_string("}").unwrap());

        assert_eq!(Token::TypeStart, Token::token_for_string("(").unwrap());
        assert_eq!(Token::TypeEnd, Token::token_for_string(")").unwrap());

        assert_eq!(Token::Eof, Token::token_for_string("\n").unwrap());

        assert_eq!(Token::MemoryStart, Token::token_for_string("[").unwrap());
        assert_eq!(Token::MemoryEnd, Token::token_for_string("]").unwrap());

        assert_eq!(Token::Assignment, Token::token_for_string("=").unwrap());
    }

    #[test]
    fn token_for_number_int() {
        Assert::int_token(&Token::token_for_number("1"), 1);
        Assert::int_token(&Token::token_for_number("2"), 2);
        Assert::int_token(&Token::token_for_number("0"), 0);
        Assert::int_token(&Token::token_for_number("129"), 129);
        Assert::int_token(&Token::token_for_number("9223372036854775807"), std::i64::MAX);
    }

    #[test]
    fn token_for_number_float() {
        Assert::float_token(&Token::token_for_number("1.0"), 1.0);
        Assert::float_token(&Token::token_for_number("2.0"), 2.0);
        Assert::float_token(&Token::token_for_number("0.0"), 0.0);
        Assert::float_token(&Token::token_for_number("129.0"), 129.0);

        Assert::float_token(&Token::token_for_number("1.10"), 1.1);
        Assert::float_token(&Token::token_for_number("2.10"), 2.1);
        Assert::float_token(&Token::token_for_number("0.10"), 0.1);
        Assert::float_token(&Token::token_for_number("129.10"), 129.1);

        Assert::float_token(&Token::token_for_number("1.0010"), 1.001);
        Assert::float_token(&Token::token_for_number("2.0020"), 2.002);
        Assert::float_token(&Token::token_for_number("0.0030"), 0.003);
        Assert::float_token(&Token::token_for_number("129.0040"), 129.004);
        Assert::float_token(&Token::token_for_number("129.0040000000000000000000"), 129.004);
        //        Assert::float_token(&Token::token_for_number("9223372036854775807").unwrap(), std::i64::MAX);
        //        Assert::float_token(&Tokenizer::tokenize("807.00.0001")[0], 807.000001);
    }

    #[test]
    fn token_for_number_nan() {
        assert_eq!(Token::NaN, Token::token_for_number("1.1.1"));
        assert_eq!(Token::NaN, Token::token_for_number("1.10.1"));
        assert_eq!(Token::NaN, Token::token_for_number("0.10.1"));
        assert_eq!(Token::NaN, Token::token_for_number("NaN"));
    }
}
