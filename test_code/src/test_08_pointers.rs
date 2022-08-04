use crate::test_08_pointers::List::{Cons, Nil};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub fn string_box_tests() {
    let mut string_pointer = Box::new(String::from("foobar"));
    println!("{}", string_pointer);
    *string_pointer += "bar!";
    println!("{}", string_pointer);

    let string = *string_pointer;
    println!("{}", string);

    // This fails, because the value has been moved:
    // println!("{}", string_pointer);
}

enum List {
    Cons(u32, Rc<List>),
    Nil,
}

impl List {
    fn new(values: &[u32]) -> List {
        if values.len() == 0 {
            Nil
        } else {
            Cons(values[0], Rc::new(Self::new(&values[1..])))
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Nil => write!(f, "[]"),
            Cons(v, tail) => {
                let mut result = String::from(format!("[{}", v));

                // TODO: is there better ways to do this?
                let mut tail = &**tail;

                let mut i = 1;
                while let Cons(v, new_tail) = tail {
                    if i >= 10 {
                        result.push_str(", ...");
                        break;
                    }
                    i += 1;

                    result.push_str(&format!(", {}", v));
                    tail = &**new_tail;
                }

                result.push_str("]");

                write!(f, "{}", result)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        match self {
            Cons(v, _) => println!("Dropping value: {}", v),
            Nil => println!("Dropping Nil"),
        }
    }
}

pub fn main() {
    string_box_tests();

    let mut list = Rc::new(List::new(&vec![]));
    let mut list_3: Option<Rc<List>> = None;

    for i in 0..12 {
        println!("{}", list);
        list = Rc::new(Cons(i, list));

        if i == 3 {
            list_3 = Option::Some(Rc::clone(&list));
        }
    }

    let list_3 = list_3.unwrap();

    println!("{}", list);
    println!("{}", list_3);
    println!("{}", Cons(42, Rc::clone(&list_3)));

    println!("list_3 strong count: {}", Rc::strong_count(&list_3));

    println!("Dropping list");
    drop(list);
    println!("list_3 strong count: {}", Rc::strong_count(&list_3));
    println!("Dropping list_3");
    drop(list_3);
}
