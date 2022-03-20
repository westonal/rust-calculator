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
    operator_stack: Vec<(T, ShuntOperator)>,
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

struct ShuntOperator {
    associativity: Associativity,
    precedence: u8,
}

pub enum ShuntType {
    Operand,
    Operator {
        associativity: Associativity,
        precedence: u8,
    },
}

pub trait ShuntingYardToken {
    fn shunt_type(&self) -> ShuntType;
}

impl<T: ShuntingYardToken> ShuntingYard<T> {
    pub fn new() -> Self {
        Self {
            output_queue: Default::default(),
            operator_stack: Default::default(),
        }
    }

    pub fn push(&mut self, token: T) {
        let shunt_type = token.shunt_type();
        match shunt_type {
            ShuntType::Operand => self.output_queue.push_back(token),
            ShuntType::Operator { associativity: o1associativity, precedence: o1precedence } => {
                while !self.operator_stack.is_empty() {
                    let (_, operator2) = self.operator_stack.last().unwrap();
                    let o2associativity = &operator2.associativity;
                    let o2precedence = &operator2.precedence;
                    if o2precedence > &o1precedence || o2precedence == &o1precedence && o2associativity == &Associativity::Left {
                        let (token, _) = self.operator_stack.pop().unwrap();
                        self.output_queue.push_back(token);
                    } else {
                        break;
                    }
                }
                self.operator_stack.push((token, ShuntOperator { associativity: o1associativity, precedence: o1precedence }));
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
                _ => ShuntType::Operand
            }
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
        yard.push("2");
        assert_eq!(vec!["1", "2"], yard.into_vec());
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
}