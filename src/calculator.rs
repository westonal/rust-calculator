use std::collections::VecDeque;
use std::str::FromStr;

use crate::math::Math;
use crate::shunting_yard::Shunt;
use crate::tokenizer::{Token, Tokenize};

pub struct Calculator {}

enum ParsedToken<T, S> {
    Operand(T),
    Operator(S),
}

struct Memory<T> {
    stack: VecDeque<T>,
}

pub trait ParseOperand {
    fn parse_operand<F: FromStrValue>(self) -> Result<F, F::Err>;
}

pub trait FromStrValue: FromStr {
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl ParseOperand for &str {
    fn parse_operand<F: FromStrValue>(self) -> Result<F, F::Err> {
        FromStrValue::from_str(self)
    }
}

impl FromStrValue for i32 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }
}

impl FromStrValue for f64 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tau" => Ok(std::f64::consts::TAU),
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            _ => s.parse()
        }
    }
}

impl<T> Memory<T> {
    fn new() -> Self {
        Self {
            stack: VecDeque::new()
        }
    }

    fn push(&mut self, t: T) {
        self.stack.push_back(t);
    }

    fn push_operator(&mut self, t: Token) -> Result<(), String>
        where T: FromStrValue + Math<T>
    {
        let n = Self::operand_count(&t);
        let mut top_n = self.pop_n(n)?;

        let left = top_n.pop().ok_or("No lhs");
        let right = top_n.pop().ok_or("No rhs");

        let value = match t {
            Token::T(_) => panic!(),
            Token::Plus => left? + right?,
            Token::Minus => left? - right?,
            Token::Multiply => left? * right?,
            Token::Divide => left? / right?,
            Token::Percent => left?.percent(),
            Token::Power => left?.pow(right?),
            Token::Root => left?.root(right?),
            // TODO: have different set of tokens for input and output?
            //  Pain because will have to map them. This might be the cleanest solution
            Token::OpenBrace => panic!(),
            Token::CloseBrace => panic!(),
        };

        self.stack.push_back(value);

        Ok(())
    }

    fn top(mut self) -> Result<T, String> {
        self.stack.pop_back().ok_or_else(|| "Empty stack".to_string())
    }
    fn pop_n(&mut self, n: u8) -> Result<Vec<T>, String> {
        (0..n).map(|_| self.stack.pop_back().ok_or_else(|| "Empty stack".to_string())).collect()
    }

    fn operand_count(t: &Token) -> u8 {
        match t {
            Token::T(_) => { panic!() }
            Token::Plus => 2,
            Token::Minus => 2,
            Token::Multiply => 2,
            Token::Divide => 2,
            Token::Percent => 1,
            Token::Power => 2,
            Token::Root => 2,
            // TODO: have different set of tokens for input and output?
            //  Pain because will have to map them. This might be the cleanest solution
            Token::OpenBrace => { panic!() }
            Token::CloseBrace => { panic!() }
        }
    }
}

impl Calculator {
    pub(crate) fn calculate<T>(&self, expression: &str) -> Result<T, String>
        where T: FromStrValue + Math<T>
    {
        let map =
            expression
                .chars()
                .tokenize()
                .shunt()
                .map(|t| {
                    match t {
                        Token::T(v) => {
                            let result: Result<T, String> = v.parse_operand::<T>().map_err(|_| format!("Cannot parse \"{}\"", v));
                            result.map(|f| ParsedToken::Operand(f))
                        }
                        _ => {
                            Ok(ParsedToken::Operator(t))
                        }
                    }
                })
                .collect::<Result<Vec<ParsedToken<T, Token>>, String>>();

        let memory = map?.into_iter()
            .fold(Ok(Memory::new()),
                  |mut memory: Result<Memory<T>, String>, token| {
                      if let Ok(memory) = memory.as_mut() {
                          match token {
                              ParsedToken::Operand(operand) => memory.push(operand),
                              ParsedToken::Operator(o) => memory.push_operator(o)?,
                          }
                      }
                      memory
                  })?;
        memory.top()
    }
}

#[cfg(test)]
mod calculator_tests {
    use super::*;

    #[test]
    fn constant() {
        assert_eq!(Ok(1), Calculator {}.calculate("1"));
    }

    #[test]
    pub fn add() {
        assert_eq!(Ok(3), Calculator {}.calculate("1+2"));
    }

    #[test]
    pub fn subtract() {
        assert_eq!(Ok(2), Calculator {}.calculate("5-3"));
    }

    #[test]
    pub fn multiply() {
        assert_eq!(Ok(18), Calculator {}.calculate("3*6"));
    }

    #[test]
    pub fn subtract_into_negative() {
        assert_eq!(Ok(-1i32), Calculator {}.calculate("2-3"));
    }

    #[test]
    pub fn integer_divide() {
        assert_eq!(Ok(3), Calculator {}.calculate("7/2"));
    }

    #[test]
    pub fn floating_point_divide() {
        assert_eq!(Ok(3.5), Calculator {}.calculate("7/2"));
    }

    #[test]
    pub fn unnecessary_brackets() {
        assert_eq!(Ok(7), Calculator {}.calculate("1+(3*2)"));
    }

    #[test]
    pub fn brackets_changing_precedence() {
        assert_eq!(Ok(8), Calculator {}.calculate("(1+3)*2"));
    }

    #[test]
    pub fn two_braces() {
        assert_eq!(Ok(12), Calculator {}.calculate("(1+3)*(5-2)"));
    }

    #[test]
    pub fn power() {
        assert_eq!(Ok(16), Calculator {}.calculate("2^4"));
    }

    #[test]
    pub fn root() {
        assert_eq!(Ok(4f64), Calculator {}.calculate("2√16"));
        assert_eq!(Ok(3f64), Calculator {}.calculate("3√27"));
    }

    #[test]
    pub fn percent() {
        assert_eq!(Ok(0.95), Calculator {}.calculate("95%"));
    }

    #[test]
    pub fn power_with_right_multiplier() {
        let calculator = Calculator {};
        assert_eq!(calculator.calculate::<i32>("(2^4)*2"), calculator.calculate("2^4*2"))
    }

    #[test]
    pub fn power_with_left_multiplier() {
        let calculator = Calculator {};
        assert_eq!(calculator.calculate::<i32>("3*(2^4)"), calculator.calculate("3*2^4"))
    }

    #[test]
    pub fn tau() {
        assert_eq!(Ok(std::f64::consts::TAU), Calculator {}.calculate("tau"))
    }

    #[test]
    pub fn pi() {
        assert_eq!(Ok(std::f64::consts::PI), Calculator {}.calculate("pi"))
    }

    #[test]
    pub fn e() {
        assert_eq!(Ok(std::f64::consts::E), Calculator {}.calculate("e"))
    }

    #[test]
    pub fn automatic_multiplication() {
        assert_eq!(Ok(12), Calculator {}.calculate("(3)(4)"));
        assert_eq!(Ok(12), Calculator {}.calculate("(3)4"));
        assert_eq!(Ok(12), Calculator {}.calculate("3(4)"));
        assert_eq!(Ok(24), Calculator {}.calculate("(2)3(4)"));
        assert_eq!(Ok(5), Calculator {}.calculate("(2)3-1"));
    }

    #[test]
    pub fn multiplication_by_constant() {
        assert_eq!(Ok(std::f64::consts::TAU), Calculator {}.calculate("2pi"))
    }
}
