use core::fmt;
use core::fmt::Formatter;
use std::collections::VecDeque;
use std::str::Chars;
use crate::shunting_yard::{Associativity, ShuntingYardToken, ShuntType};

enum Mode {
    None,

}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    T(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    OpenBrace,
    CloseBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::T(s) => f.write_str(s),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Multiply => f.write_str("*"),
            Token::Divide => f.write_str("/"),
            Token::Power => f.write_str("^"),
            Token::OpenBrace => f.write_str("("),
            Token::CloseBrace => f.write_str(")"),
        }
    }
}

struct TokenizerState {
    mode: Mode,
    tokens: VecDeque<Token>,
    current_token: Vec<char>,
}

impl TokenizerState {
    fn new() -> TokenizerState {
        TokenizerState {
            mode: Mode::None,
            tokens: Default::default(),
            current_token: vec![],
        }
    }

    fn complete(mut self) -> Vec<Token> {
        self.end_node();
        self.tokens.into()
    }

    fn end_node(&mut self) {
        let contents = std::mem::take(&mut self.current_token);
        if !contents.is_empty() {
            let string: String = contents.into_iter().collect();
            // TODO: annoying repeat 1/2
            if string.as_str() == "+" {
                self.tokens.push_back(Token::Plus)
            } else if string.as_str() == "-" {
                self.tokens.push_back(Token::Minus)
            } else if string.as_str() == "(" {
                self.tokens.push_back(Token::OpenBrace)
            } else if string.as_str() == ")" {
                self.tokens.push_back(Token::CloseBrace)
            } else if string.as_str() == "*" || string.as_str() == "x" {
                self.tokens.push_back(Token::Multiply)
            } else if string.as_str() == "/" {
                self.tokens.push_back(Token::Divide)
            } else if string.as_str() == "^" {
                self.tokens.push_back(Token::Power)
            } else {
                self.tokens.push_back(Token::T(string));
            }
        }
    }

    fn push(&mut self, expression: &str) {
        expression.chars().for_each(|c| {
            self.push_char(c)
        })
    }

    fn push_char(&mut self, c: char) {
        match self.mode {
            Mode::None => {
                // TODO: Annoying repeat 2/2
                if c == '+' || c == '-' || c == '*' || c == 'x' || c == '/' || c == '^' || c == '(' || c == ')' {
                    self.end_node();
                    self.current_token.push(c);
                    self.end_node();
                } else {
                    self.current_token.push(c);
                }
            }
        }
    }
}

pub struct TokenStream<I: Iterator<Item=char>> {
    iter: I,
    state: TokenizerState,
}

impl<I: Iterator<Item=char>> TokenStream<I> {
    fn new(iter: I) -> TokenStream<I> {
        Self {
            iter,
            state: TokenizerState::new(),
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for TokenStream<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let head = self.state.tokens.pop_front();
            if head.is_some() {
                return head;
            }
            if let Some(c) = self.iter.next() {
                self.state.push_char(c);
            } else {
                self.state.end_node();
                if self.state.tokens.is_empty() {
                    return None;
                }
            }
        }
    }
}

pub trait Tokenize<R: Iterator<Item=Token>> {
    fn tokenize(self) -> R;
}

impl<I: Iterator<Item=char>> Tokenize<TokenStream<I>> for I {
    fn tokenize(self) -> TokenStream<I> {
        TokenStream::new(self)
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn number() {
        let tokens = "1".chars().into_iter().tokenize();
        let map: Vec<Token> = tokens.collect();
        assert_eq!(1, map.len());
        assert_eq!(Some(&Token::T("1".to_string())), map.get(0))
    }

    #[test]
    fn plus() {
        expect_token("+", &Token::Plus);
    }

    #[test]
    fn minus() {
        expect_token("-", &Token::Minus);
    }

    #[test]
    fn multiply() {
        expect_token("*", &Token::Multiply);
    }

    #[test]
    fn multiply_with_lower_case_x() {
        expect_token("x", &Token::Multiply);
    }

    #[test]
    fn divide() {
        expect_token("/", &Token::Divide);
    }

    #[test]
    fn power() {
        expect_token("^", &Token::Power);
    }

    #[test]
    fn open_brace() {
        expect_token("(", &Token::OpenBrace);
    }

    #[test]
    fn close_brace() {
        expect_token(")", &Token::CloseBrace);
    }

    fn expect_token(input: &str, expected: &Token) {
        let tokens = input.chars().into_iter().tokenize();
        let map: Vec<Token> = tokens.into_iter().collect();
        assert_eq!(1, map.len());
        assert_eq!(Some(expected), map.get(0));
    }

    #[test]
    fn short_numbers_and_plus() {
        let tokens = "1+2".chars().into_iter().tokenize();
        let map: Vec<String> = tokens.into_iter().map(|t| { t.to_string() }).collect();
        assert_eq!(vec!["1", "+", "2"], map);
    }

    #[test]
    fn longer_numbers_and_plus() {
        let tokens = "123+456".chars().into_iter().tokenize();
        let map: Vec<String> = tokens.into_iter().map(|t| { t.to_string() }).collect();
        assert_eq!(vec!["123", "+", "456"], map);
    }
}

impl ShuntingYardToken for Token {
    fn shunt_type(&self) -> ShuntType {
        match &self {
            Token::T(_) => ShuntType::Operand,
            Token::Plus => ShuntType::Operator { associativity: Associativity::Left, precedence: 0 },
            Token::Minus => ShuntType::Operator { associativity: Associativity::Left, precedence: 0 },
            Token::Multiply => ShuntType::Operator { associativity: Associativity::Left, precedence: 1 },
            Token::Divide => ShuntType::Operator { associativity: Associativity::Left, precedence: 1 },
            Token::Power => ShuntType::Operator { associativity: Associativity::Left, precedence: 2 },
            Token::OpenBrace => ShuntType::OpenBrace,
            Token::CloseBrace => ShuntType::CloseBrace,
        }
    }
}

#[cfg(test)]
mod shunting_yard_integration_tests {
    use crate::shunting_yard::Shunt;
    use crate::tokenizer::Token::*;
    use super::*;

    #[test]
    fn a() {
        let tokens = "123+456*12".chars().into_iter().tokenize().shunt();
        assert_eq!(vec![
            T("123".to_string()),
            T("456".to_string()),
            T("12".to_string()),
            Multiply,
            Plus,
        ], tokens.collect::<Vec<Token>>());
    }
}

#[cfg(test)]
mod tokenizer_state_test {
    use crate::tokenizer::Token::{Plus, T};
    use crate::tokenizer::TokenizerState;

    #[test]
    fn single_char() {
        let mut tokenizer = TokenizerState::new();
        tokenizer.push_char('1');
        let tokens = tokenizer.complete();
        assert_eq!(vec![T("1".to_string())], tokens);
    }

    #[test]
    fn two_chars() {
        let mut tokenizer = TokenizerState::new();
        tokenizer.push_char('1');
        tokenizer.push_char('2');
        let tokens = tokenizer.complete();
        assert_eq!(vec![T("12".to_string())], tokens);
    }

    #[test]
    fn two_chars_with_separator() {
        let mut tokenizer = TokenizerState::new();
        tokenizer.push_char('1');
        tokenizer.push_char('+');
        tokenizer.push_char('2');
        let tokens = tokenizer.complete();
        assert_eq!(vec![
            T("1".to_string()),
            Plus,
            T("2".to_string()),
        ], tokens);
    }
}
