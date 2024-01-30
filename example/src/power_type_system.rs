use std::fmt::Display;

struct Example<'a> {
    message: &'a str,
}

impl<'a> Display for Example<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

trait Find<P, T>
where
    P: Fn(&T) -> bool,
{
    fn new_find_func(&self, predicate: P) -> Option<&T>;
}

impl<P, T> Find<P, T> for Vec<T>
where
    P: Fn(&T) -> bool,
{
    fn new_find_func(&self, predicate: P) -> Option<&T> {
        self.iter().filter(|p| predicate(*p)).take(1).next()
    }
}

pub fn main() {
    let list = vec![1, 2, 3, 4, 5];
    println!("{:?}", list.new_find_func(|i| *i > 3));
    println!("{:?}", list.new_find_func(|i| *i == 10));
}
