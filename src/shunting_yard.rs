use std::collections::vec_deque::IntoIter;
use std::collections::VecDeque;
use std::mem;

pub struct Shunted<T: ShuntingYardToken, I: Iterator<Item=T>> {
    iter: I,
    yard: ShuntingYard<T>,
}

impl<T: ShuntingYardToken, I: Iterator<Item=T>> Iterator for Shunted<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let front = self.yard.output_queue.pop_front();
        if front.is_some() {
            return front;
        }
        loop {
            let input = self.iter.next();
            match input {
                None => {
                    self.yard.end();
                    return self.yard.output_queue.pop_front();
                }
                Some(next) => { self.yard.push(next); }
            };
            let front = self.yard.output_queue.pop_front();
            if front.is_some() {
                return front;
            }
        }
    }
}

pub trait Shunt<T: ShuntingYardToken, I: Iterator<Item=T>>: Iterator<Item=T> {
    fn shunt(self) -> Shunted<T, I>;
}

impl<T: ShuntingYardToken, I: Iterator<Item=T>> Shunt<T, I> for I {
    fn shunt(self) -> Shunted<T, I> {
        Shunted {
            iter: self,
            yard: ShuntingYard::new(),
        }
    }
}

pub struct ShuntingYard<T: ShuntingYardToken> {
    output_queue: VecDeque<T>,
    operator_stack: Vec<(T, ShuntType)>,
    last_shunt_type: Option<ShuntType>,
}

impl<T: ShuntingYardToken> Default for ShuntingYard<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq)]
pub enum Associativity {
    Left
}

pub enum ShuntType {
    Operand,
    Operator {
        associativity: Associativity,
        precedence: u8,
    },
    OpenBrace,
    CloseBrace,
}

pub trait ShuntingYardToken: Sized {
    fn shunt_type(&self) -> ShuntType;
    /// When two operands have no token between them, this is the token that should be assumed
    fn operand_separator() -> Option<Self>;
}

impl<T: ShuntingYardToken> ShuntingYard<T> {
    pub fn new() -> Self {
        Self {
            output_queue: Default::default(),
            operator_stack: Default::default(),
            last_shunt_type: Default::default(),
        }
    }

    pub fn push(&mut self, token: T) {
        let shunt_type = token.shunt_type();
        match &shunt_type {
            ShuntType::Operand => {
                let previous_shunt_type = self.last_shunt_type.replace(shunt_type);
                self.output_queue.push_back(token);
                if let Some(ShuntType::Operand) = previous_shunt_type {
                    if let Some(injected_separator_token) = <T as ShuntingYardToken>::operand_separator() {
                        let injected_shunt_type = injected_separator_token.shunt_type();
                        self.operator_stack.push((injected_separator_token, injected_shunt_type));
                    }
                }
            }
            ShuntType::Operator { associativity: _o1associativity, precedence: o1precedence } => {
                self.last_shunt_type = None;
                while !self.operator_stack.is_empty() {
                    let (_, operator2) = self.operator_stack.last().unwrap();
                    match operator2 {
                        ShuntType::Operand => panic!("Operand in operator stack"),
                        ShuntType::Operator { associativity: o2associativity, precedence: o2precedence } => {
                            if o2precedence > o1precedence || o2precedence == o1precedence && o2associativity == &Associativity::Left {
                                let (token, _) = self.operator_stack.pop().unwrap();
                                self.output_queue.push_back(token);
                            } else {
                                break;
                            }
                        }
                        ShuntType::OpenBrace => {
                            break;
                        }
                        ShuntType::CloseBrace => panic!("CloseBrace in operator stack"),
                    }
                }
                self.operator_stack.push((token, shunt_type));
            }
            ShuntType::OpenBrace => {
                self.operator_stack.push((token, shunt_type))
            }
            ShuntType::CloseBrace => {
                while !self.operator_stack.is_empty() {
                    let (_, operator2) = self.operator_stack.last().unwrap();
                    if let ShuntType::OpenBrace = operator2 {
                        // Discard
                        self.operator_stack.pop();
                        break;
                    }
                    let (token, _) = self.operator_stack.pop().unwrap();
                    self.output_queue.push_back(token);
                    // TODO, should stop on '(', also understand where that goes... not disposed here

                    //while the operator at the top of the operator stack is not a left parenthesis:
                    //             {assert the operator stack is not empty}
                    //             /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
                    //             pop the operator from the operator stack into the output queue
                    //         {assert there is a left parenthesis at the top of the operator stack}
                    //         pop the left parenthesis from the operator stack and discard it
                    //         if there is a function token at the top of the operator stack, then:
                    //             pop the function from the operator stack into the output queue
                }
            }
        }
    }

    fn end(&mut self) {
        if !self.operator_stack.is_empty() {
            let mut remaining_operator_tokens: VecDeque<T> = mem::take(&mut self.operator_stack)
                .into_iter()
                .map(|(token, _)| token)
                .rev()
                .collect();
            self.output_queue.append(&mut remaining_operator_tokens);
        }
    }

    /// Consumes the `ShuntingYard` into a reverse polish notation iterator
    pub(crate) fn into_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }
}

impl<T: ShuntingYardToken> From<ShuntingYard<T>> for Vec<T> {
    fn from(mut yard: ShuntingYard<T>) -> Self {
        yard.end();
        yard.output_queue.into_iter().collect()
    }
}

impl<T: ShuntingYardToken> IntoIterator for ShuntingYard<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the `ShuntingYard` into a reverse polish notation iterator
    fn into_iter(mut self) -> Self::IntoIter {
        self.end();
        self.output_queue.into_iter()
    }
}

#[cfg(test)]
mod shunting_yard_tests {
    use super::*;

    impl ShuntingYardToken for &str {
        fn shunt_type(&self) -> ShuntType {
            match *self {
                "+" => ShuntType::Operator { associativity: Associativity::Left, precedence: 0 },
                "-" => ShuntType::Operator { associativity: Associativity::Left, precedence: 0 },
                "*" => ShuntType::Operator { associativity: Associativity::Left, precedence: 1 },
                "(" => ShuntType::OpenBrace,
                ")" => ShuntType::CloseBrace,
                _ => ShuntType::Operand
            }
        }

        fn operand_separator() -> Option<Self> {
            Some("*")
        }
    }

    #[test]
    fn single_token() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        assert_eq!(vec!["1"], yard.into_vec());
    }

    #[test]
    fn two_tokens() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("+");
        assert_eq!(vec!["1", "+"], yard.into_vec());
    }

    #[test]
    fn two_tokens_and_operator() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("+");
        yard.push("2");
        assert_eq!(vec!["1", "2", "+"], yard.into_vec());
    }

    #[test]
    fn two_operators_same_precedence() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("+");
        yard.push("2");
        yard.push("-");
        yard.push("3");
        assert_eq!(vec!["1", "2", "+", "3", "-"], yard.into_vec());
    }

    #[test]
    fn two_operators_different_precedence() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("*");
        yard.push("2");
        yard.push("+");
        yard.push("3");
        assert_eq!(vec!["1", "2", "*", "3", "+"], yard.into_vec());
    }

    #[test]
    fn two_operators_different_precedence_other_order() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("+");
        yard.push("2");
        yard.push("*");
        yard.push("3");
        assert_eq!(vec!["1", "2", "3", "*", "+"], yard.into_vec());
    }

    #[test]
    /// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    fn wiki_example() {
        let mut yard = ShuntingYard::new();
        yard.push("A");
        yard.push("+");
        yard.push("B");
        yard.push("*");
        yard.push("C");
        yard.push("-");
        yard.push("D");
        assert_eq!(vec!["A", "B", "C", "*", "+", "D", "-"], yard.into_vec());
    }

    #[test]
    fn unnecessary_bracket_test() {
        let mut yard = ShuntingYard::new();
        yard.push("1");
        yard.push("+");
        yard.push("(");
        yard.push("2");
        yard.push("*");
        yard.push("3");
        yard.push(")");
        assert_eq!(vec!["1", "2", "3", "*", "+"], yard.into_vec());
    }

    #[test]
    fn bracket_test() {
        let mut yard = ShuntingYard::new();
        yard.push("(");
        yard.push("1");
        yard.push("+");
        yard.push("2");
        yard.push(")");
        yard.push("*");
        yard.push("3");
        assert_eq!(vec!["1", "2", "+", "3", "*"], yard.into_vec());
    }

    #[test]
    fn bracket_test_preceding_operators() {
        let mut yard = ShuntingYard::new();
        yard.push("3");
        yard.push("*");
        yard.push("(");
        yard.push("1");
        yard.push("+");
        yard.push("2");
        yard.push(")");
        assert_eq!(vec!["3", "1", "2", "+", "*"], yard.into_vec());
    }

    #[test]
    fn automatic_multiplication_of_adjacent_operands() {
        let mut yard = ShuntingYard::new();
        yard.push("3");
        yard.push("4");
        assert_eq!(vec!["3", "4", "*"], yard.into_vec());
    }
}