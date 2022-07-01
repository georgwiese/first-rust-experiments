struct Color(u8, u8, u8);

struct User {
    username: String,
    email: String,
    active: bool,
    color: Color,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn make_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        color: Color(0, 0, 0),
    }
}

pub fn main() {
    let mut user = make_user(String::from("georg"), String::from("georgwiese@gmail.com"));

    user.active = false;

    println!("User name: {}", user.username);
    println!("Email:     {}", user.email);
    println!("Active:    {}", user.active);
    println!(
        "Color:     ({}, {}, {})",
        user.color.0, user.color.1, user.color.2
    );
    println!();

    let rect = Rectangle {
        width: 10,
        height: 30,
    };
    println!("Rectangle:    {:?}", rect);
    println!("Area:         {}", rect.area());
    println!("Can hold (1): {}", rect.can_hold(&Rectangle::square(10)));
    println!("Can hold (2): {}", rect.can_hold(&Rectangle::square(30)));
}
