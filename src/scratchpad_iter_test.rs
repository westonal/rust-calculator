
struct A {}

impl Iterator for A {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let take: Vec<i32> = A {}.into_iter().take(4).collect();
        assert_eq!(vec![7, 7, 7, 7], take);
    }

    #[test]
    fn b() {
        let f: Vec<char> = "abc".to_string().chars().into_iter().take(2).collect();
        assert_eq!(vec!['a', 'b'], f)
    }

    // #[test]
    // fn stringify_char() {
    //     let f: Vec<char> = "abc".to_string().chars().into_iter().stringify().collect();
    //     assert_eq!(vec!['a', 'b'], f)
    // }
}

trait Stringify<T: Iterator<Item=char> + Sized>: Iterator<Item=char> + Sized {
    fn stringify(self) -> T {
        todo!()
    }
}

impl<T: Iterator<Item=char> + Sized> Stringify<T> for T {}

//
// struct TokenI2<I:Iterator<Item=char>> {
//     //iter: I,
//
// }
//
// impl<I:Iterator<Item=char>> Iterator for TokenI2<I>{
//     type Item = String;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
