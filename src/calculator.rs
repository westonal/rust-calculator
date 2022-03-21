use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
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

impl<T> Memory<T> {
    fn new() -> Self {
        Self {
            stack: VecDeque::new()
        }
    }

    fn push(&mut self, t: T)
        where T: FromStr + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
    {
        self.stack.push_back(t);
    }

    fn push_operator(&mut self, t: Token) -> Result<(), String>
        where T: FromStr + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
    {
        let right = self.stack.pop_back().ok_or("No rhs")?;
        let left = self.stack.pop_back().ok_or("No lhs")?;
        match t {
            Token::T(_) => { panic!() }
            Token::Plus => { self.stack.push_back(left + right); }
            Token::Minus => { self.stack.push_back(left - right); }
            Token::Multiply => { self.stack.push_back(left * right); }
            Token::Divide => { self.stack.push_back(left / right); }
            // TODO: have different set of tokens for input and output?
            //  Pain because will have to map them. This might be the cleanest solution
            Token::OpenBrace => { panic!() }
            Token::CloseBrace => { panic!() }
        };
        Ok(())
    }

    fn top(mut self) -> Result<T, String> {
        self.stack.pop_back().ok_or("Empty stack".to_string()) // TODO: understand why to_string here but not above in "No rhs"/"no lhs"?
    }
}

impl Calculator {
    pub(crate) fn calculate<T>(&self, expression: &str) -> Result<T, String>
        where T: FromStr + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
    {
        let map =
            expression
                .chars()
                .tokenize()
                .shunt()
                .map(|t| {
                    match t {
                        Token::T(v) => {
                            let result: Result<T, String> = v.parse::<T>().map_err(|_| format!("Cannot parse \"{}\"", v));
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
