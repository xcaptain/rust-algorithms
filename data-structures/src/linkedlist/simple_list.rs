// http://cglab.ca/~abeinges/blah/too-many-lists/book/
// https://doc.rust-lang.org/book/second-edition/ch15-01-box.html
// https://rustbyexample.com/custom_types/enum/testcase_linked_list.html

use self::SimpleList::{Cons, Nil};

pub enum SimpleList {
    Cons(u32, Box<SimpleList>),
    Nil,
}

impl SimpleList {
    pub fn new() -> SimpleList {
        Nil
    }

    pub fn prepend(self, elem: u32) -> SimpleList {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_list() {
        let l = SimpleList::new();
        assert_eq!(0, l.len());
    }

    #[test]
    fn test_prepend() {
        let mut l = SimpleList::new();
        l = l.prepend(10);
        assert_eq!(1, l.len());
    }

    #[test]
    fn test_stringify() {
        let mut l = SimpleList::new();
        l = l.prepend(10);
        assert_eq!(String::from("10, Nil"), l.stringify());
    }
}
