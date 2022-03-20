use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use crate::shunting_yard::Shunt;
use crate::tokenizer::{Token, Tokenize};

pub struct Calculator {}

impl Calculator {
    pub(crate) fn calculate<T: FromStr + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>>(&self, expression: &str) -> T {
        let shunted = expression.chars().tokenize().shunt();

        let mut stack = VecDeque::new();
        shunted.into_iter().for_each(|t| {
            match t {
                Token::T(v) => {
                    stack.push_back(v.parse::<T>().unwrap_or_else(|_| panic!()));
                }
                _ => {
                    let right = stack.pop_back().unwrap();
                    let left = stack.pop_back().unwrap();
                    match t {
                        Token::T(_) => { panic!() }
                        Token::Plus => { stack.push_back(left + right); }
                        Token::Minus => { stack.push_back(left - right); }
                        Token::Multiply => { stack.push_back(left * right); }
                        Token::Divide => { stack.push_back(left / right); }
                        // TODO: have different set of tokens for input and output?
                        //  Pain because will have to map them. This might be the cleanest solution
                        Token::OpenBrace => { panic!() }
                        Token::CloseBrace => { panic!() }
                    }
                }
            }
        });

        stack.pop_back().unwrap()
    }
}

#[cfg(test)]
mod calculator_tests {
    use crate::calculator::Calculator;

    #[test]
    fn constant() {
        assert_eq!(1, Calculator {}.calculate("1"))
    }

    #[test]
    pub fn add() {
        assert_eq!(3, Calculator {}.calculate("1+2"))
    }

    #[test]
    pub fn subtract() {
        assert_eq!(2, Calculator {}.calculate("5-3"))
    }

    #[test]
    pub fn multiply() {
        assert_eq!(18, Calculator {}.calculate("3*6"))
    }

    #[test]
    pub fn subtract_into_negative() {
        assert_eq!(-1, Calculator {}.calculate("2-3"))
    }

    #[test]
    pub fn integer_divide() {
        assert_eq!(3, Calculator {}.calculate("7/2"))
    }

    #[test]
    pub fn floating_point_divide() {
        assert_eq!(3.5, Calculator {}.calculate("7/2"))
    }

    #[test]
    pub fn unnecessary_brackets() {
        assert_eq!(7, Calculator {}.calculate("1+(3*2)"))
    }

    #[test]
    pub fn brackets_changing_precedence() {
        assert_eq!(8, Calculator {}.calculate("(1+3)*2"))
    }

    #[test]
    pub fn two_braces() {
        assert_eq!(12, Calculator {}.calculate("(1+3)*(5-2)"))
    }
}
