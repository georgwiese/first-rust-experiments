use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation<T>(parameter: &T) -> T
where
    T: Display,
    T: Add<Output = T>,
    T: Copy,
{
    println!("Calculating: {}", parameter);
    thread::sleep(Duration::from_secs(2));
    return parameter.clone() + *parameter;
}

// TODO: How to make simulated_expensive_calculation work for strings too?
fn simulated_expensive_calculation_string(parameter: &String) -> String {
    println!("Calculating: {}", parameter);
    thread::sleep(Duration::from_secs(2));
    return parameter.clone() + parameter;
}

struct Cacher<T, P, R>
where
    T: Fn(&P) -> R,
    R: Clone,
    P: Hash,
    P: Eq,
{
    values: HashMap<P, R>,
    calculation: T,
}

impl<T, P, R> Cacher<T, P, R>
where
    T: Fn(&P) -> R,
    R: Clone,
    P: Hash,
    P: Eq,
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            values: HashMap::new(),
            calculation,
        }
    }

    fn value(&mut self, parameter: P) -> R {
        match self.values.get(&parameter) {
            Some(v) => (*v).clone(),
            None => {
                let v = (self.calculation)(&parameter).clone();
                self.values.insert(parameter, v.clone());
                v
            }
        }
    }
}

pub fn main() {
    let calculation_int = |param: &u32| simulated_expensive_calculation(param);
    let mut int_cacher = Cacher::new(calculation_int);
    println!("Invocation (1): {}", int_cacher.value(1));
    println!("Invocation (2): {}", int_cacher.value(2));
    println!("Invocation (1): {}", int_cacher.value(1));
    println!("Invocation (2): {}", int_cacher.value(2));

    let calculation_string = |param: &String| simulated_expensive_calculation_string(param);
    let mut string_cacher = Cacher::new(calculation_string);
    println!(
        "Invocation (foo): {}",
        string_cacher.value(String::from("foo"))
    );
    println!(
        "Invocation (bar): {}",
        string_cacher.value(String::from("bar"))
    );
    println!(
        "Invocation (foo): {}",
        string_cacher.value(String::from("foo"))
    );
    println!(
        "Invocation (bar): {}",
        string_cacher.value(String::from("bar"))
    );
}
