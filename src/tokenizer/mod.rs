mod token;
mod assert;

//use token::Token;
//use token::Token;

use self::token::Token;

pub struct Tokenizer {}

#[derive(Debug,PartialEq)]
enum TokenizerLockMode {
    Text,
    Number,
    None,
}

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<token::Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut word = String::from("");
        let mut locked = TokenizerLockMode::None;
        // let mut token;

        for c in input.chars() {
            // if !locked && c == '"' {
            //     println!("lock");
            //     // Start of a string
            //     locked = true;
            //     continue;
            // }
//            println!("{:?}", c);

            // if locked == TokenizerLockMode::None {
            //     if c == '"' {
            //         println!("lock");
            //         // Start of a string
            //         locked = TokenizerLockMode::Text;
            //         continue;
            //     } else if c.is_numeric() {
            //         println!("consume numeric {}", c);
            //         word.push(c);
            //         locked = TokenizerLockMode::Number;
            //         continue;
            //     }
            // } else {
            //     if c != '"' {
            //         println!("consume {}", c);
            //         // Consume the char
            //         word.push(c);
            //         continue;
            //     }
            //     if c == '"' {
            //         println!("unlock");
            //         // End of a string
            //         locked = TokenizerLockMode::None;
            //
            //         tokens.push(Token::Text(word.to_owned()));
            //         word.clear();
            //         continue;
            //     }
            // }


            if locked == TokenizerLockMode::None {
                if c == '"' {
                    println!("lock text");
                    // Start of a string
                    locked = TokenizerLockMode::Text;
                    continue;
                } else if Tokenizer::is_part_of_number(c) {
                    println!("lock numeric {}", c);
                    word.push(c);
                    locked = TokenizerLockMode::Number;
                    continue;
                }
            }

            if locked == TokenizerLockMode::Number {
                if Tokenizer::is_part_of_number(c) {
                    println!("consume numeric {}", c);
                    word.push(c);

                    continue;
                } else {
                    println!("unlock number");
                    // End of a string
                    locked = TokenizerLockMode::None;

                    tokens.push(Token::token_for_number(&word));
                    word.clear();
                    continue;
                }
            }

            if locked == TokenizerLockMode::Text {
                if c != '"' {
                    println!("consume text {}", c);
                    // Consume the char
                    word.push(c);
                    continue;
                }
                if c == '"' {
                    println!("unlock text ");
                    // End of a string
                    locked = TokenizerLockMode::None;

                    tokens.push(Token::Text(word.to_owned()));
                    word.clear();
                    continue;
                }
            }

            match token::Token::token_for_char(c) {
                // If c is a token
                Some(token) => {
                    tokens.push(token);
                }

                None => {
                    if !Tokenizer::is_word_boundary(c) {
                        word.push(c);
                    } else if !word.is_empty() {
                        // Word boundary
                        println!("{:?}", Token::token_for_string(&word));

                        tokens.push((Token::token_for_string(&word)).unwrap());
                        word.clear();
                    }
                }
            }
        }

        if locked == TokenizerLockMode::Number {
            tokens.push(Token::token_for_number(&word));
        }

        if locked != TokenizerLockMode::None {
            if locked == TokenizerLockMode::Text {
                panic!("Unmatched quotes");
            }

        }
        if !word.is_empty() {
            println!("{:?}", Token::token_for_string(&word));
            tokens.push(Token::token_for_string(&word).unwrap());
        }

        tokens
    }

    fn is_word_boundary(input: char) -> bool {
        if input.is_alphabetic() {
            return false;
        }
        if input == '_' {
            return false;
        }
        return true;
    }

    fn is_part_of_number(input: char) -> bool {
        input == '.' || input.is_numeric()
    }
}

#[cfg(test)]
mod tests {
    use std;
    use super::*;
    use super::token::Token;
    use super::assert::Assert;

    #[test]
    fn tokenizer() {
        let tokens = Tokenizer::tokenize("hello my name is daniel");
        assert_eq!(5, tokens.len());

        let tokens = Tokenizer::tokenize("hello");
        assert_eq!(1, tokens.len());

        let tokens = Tokenizer::tokenize("{hello my}");
        assert_eq!(4, tokens.len());

        let tokens = Tokenizer::tokenize("{hello_my}");
        assert_eq!(3, tokens.len());
    }

    #[test]
    fn tokenize_text() {
        let tokens = Tokenizer::tokenize("\"hello\"");
        assert_eq!(1, tokens.len(), "{:?}", tokens);
        Assert::text_token(&tokens[0], "hello");

        let tokens = Tokenizer::tokenize("var = \"hello\"");
        assert_eq!(3, tokens.len(), "{:?}", tokens);
        Assert::identifier_token(&tokens[0], "var");
        assert_eq!(Token::Assignment, tokens[1]);
        Assert::text_token(&tokens[2], "hello");
    }

    #[test]
    fn tokenize_int() {
        Assert::int_token(&Tokenizer::tokenize("0")[0], 0);
        Assert::int_token(&Tokenizer::tokenize("129")[0], 129);
        Assert::int_token(&Tokenizer::tokenize("9223372036854775807")[0],
                          std::i64::MAX);
    }

    #[test]
    fn tokenize_float() {
        Assert::float_token(&Tokenizer::tokenize("0.01")[0], 0.01);
        Assert::float_token(&Tokenizer::tokenize("129.9")[0], 129.9);
        Assert::float_token(&Tokenizer::tokenize("807.000001")[0], 807.000001);
    }

    #[test]
    fn tokenize_nan() {
        assert_eq!(Token::NaN, Tokenizer::tokenize("1.1.1")[0]);
        assert_eq!(Token::NaN, Tokenizer::tokenize("807.00.0001")[0]);
        assert_eq!(Token::NaN, Tokenizer::tokenize("0.00.0001")[0]);
    }

    #[test]
    #[should_panic]
    fn tokenizer_should_fail() {
        Tokenizer::tokenize("\"hello");
    }
}
