trait SayHi {
    fn hi(&self);
}

struct GenericHi {}

impl SayHi for GenericHi {
    fn hi(&self) {
        println!("Hi!")
    }
}

struct HiWithName {
    name: String,
}

impl HiWithName {
    fn new(name: String) -> HiWithName {
        HiWithName { name }
    }
}

impl SayHi for HiWithName {
    fn hi(&self) {
        println!("Hi, {}!", self.name)
    }
}

fn polymorphism_test() {
    let his: Vec<Box<dyn SayHi>> = vec![
        Box::new(GenericHi {}),
        Box::new(HiWithName::new(String::from("Georg"))),
    ];

    for hi in his.iter() {
        hi.hi();
    }
}

pub fn main() {
    polymorphism_test();
}
