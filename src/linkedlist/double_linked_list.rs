pub struct List {
    pub head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
    }
}