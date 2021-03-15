use List::*;

pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        Nil
    }

    pub fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => tail.len() + 1,
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}
