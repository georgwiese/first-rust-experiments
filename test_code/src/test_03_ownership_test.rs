pub fn main() {
    let mut s = String::from("Foo!");

    let length = calculate_length_and_mutate(&mut s);

    // Make s immutable
    let s = s;
    // This fails now
    // calculate_length_and_mutate(&mut s);

    println!("{}, length {}", s, length);

    let first_word = first_word(&s);
    println!("{}", first_word);
}

fn calculate_length_and_mutate(s: &mut String) -> usize {
    let length = s.len();
    s.push_str(" Bar!");

    length
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
