use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use crate::shunting_yard::Shunt;
use crate::tokenizer::{Token, Tokenize};

pub struct Calculator {}

impl Calculator {
    pub(crate) fn calculate<T>(&self, expression: &str) -> Result<T, String>
    where T: FromStr + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
    {
        let mut stack = VecDeque::new();
        for token in expression.chars().tokenize().shunt() {
            match token {
                Token::T(v) => {
                    stack.push_back(v.parse::<T>().map_err(|_| format!("Cannot parse \"{}\"", v))?);
                }
                _ => {
                    let right = stack.pop_back().ok_or("No rhs")?;
                    let left = stack.pop_back().ok_or("No lhs")?;
                    match token {
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
        };

        Ok(stack.pop_back().unwrap())
    }
}

#[cfg(test)]
mod calculator_tests {
    use crate::calculator::Calculator;

    #[test]
    fn constant() {
        assert_eq!(Ok(1), Calculator {}.calculate("1"));
    }

    #[test]
    pub fn add() {
        assert_eq!(Ok(3), Calculator {}.calculate("1+2"))
    }

    #[test]
    pub fn subtract() {
        assert_eq!(Ok(2), Calculator {}.calculate("5-3"))
    }

    #[test]
    pub fn multiply() {
        assert_eq!(Ok(18), Calculator {}.calculate("3*6"))
    }

    #[test]
    pub fn subtract_into_negative() {
        assert_eq!(Ok(-1), Calculator {}.calculate("2-3"))
    }

    #[test]
    pub fn integer_divide() {
        assert_eq!(Ok(3), Calculator {}.calculate("7/2"))
    }

    #[test]
    pub fn floating_point_divide() {
        assert_eq!(Ok(3.5), Calculator {}.calculate("7/2"))
    }

    #[test]
    pub fn unnecessary_brackets() {
        assert_eq!(Ok(7), Calculator {}.calculate("1+(3*2)"))
    }

    #[test]
    pub fn brackets_changing_precedence() {
        assert_eq!(Ok(8), Calculator {}.calculate("(1+3)*2"))
    }

    #[test]
    pub fn two_braces() {
        assert_eq!(Ok(12), Calculator {}.calculate("(1+3)*(5-2)"))
    }
}
