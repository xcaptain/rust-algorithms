// http://cglab.ca/~abeinges/blah/too-many-lists/book/
// https://doc.rust-lang.org/book/second-edition/ch15-01-box.html
// https://rustbyexample.com/custom_types/enum/testcase_linked_list.html

use self::SimpleList::{Cons, Nil};

pub enum SimpleList {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<SimpleList>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl SimpleList {
    // Create an empty list
    pub fn new() -> SimpleList {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    pub fn prepend(self, elem: u32) -> SimpleList {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    pub fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
