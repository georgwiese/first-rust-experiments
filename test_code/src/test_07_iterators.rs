struct Counter {
    count: u32,
    max_count: u32,
}

impl Counter {
    fn new(max_count: u32) -> Counter {
        Counter {
            count: 0,
            max_count,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.max_count {
            return None;
        } else {
            let v = Some(self.count);
            self.count += 1;
            return v;
        }
    }
}

pub fn main() {
    let range_5: Vec<u32> = Counter::new(5).collect();
    println!("{:?}", range_5);

    let range_5_squared: Vec<u32> = Counter::new(5).map(|x| x * x).collect();
    println!("{:?}", range_5_squared);
}

#[cfg(test)]
mod tests {
    use crate::test_07_iterators::Counter;

    #[test]
    fn count_works() {
        let mut counter = Counter::new(5);
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), None);
    }
}
