fn main() {
    let s = String::from("Hello There");
    println!("String's Length is: {}", find_string_length(&s));

    // Initialising Structs
    // let someone = User {
    //     val: 23,
    //     username: String::from("r8dra"),
    //     is_married: true,
    //     age: 17
    // };

    let rect = Rect {
        x: 12,
        y: 23
    };

    println!("Area of rect is: {}", rect.area());
    // Next line will give error since implementation `static_func` of Rect struct doesn't take &self as input
    // This means they are static and must be accessed by struct name <- Kinda like static func in C++ / Java
    // println!("Trying to call static function with object name: {}", rect.static_func());

    // This works though <- when called using struct name
    println!("Trying to call static function with object name: {}", Rect::static_func());

}

// Assingment 02
fn find_string_length(s: &str) -> usize {
    s.chars().count()
}

// Assignment 01
// fn fib(x: i32) -> i32 {
//     if x == 1 {
//         0
//     } else if x == 2 {
//         1
//     } else {
//         fib(x - 1) + fib(x - 2)
//     }
// }

// Structs
// struct User {
//     val: i32,
//     username: String,
//     is_married: bool,
//     age: i32
// }

// implementing structs
struct Rect {
    x: i32,
    y: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.x * self.y
    }

    fn static_func() -> i32 {
        return 1;
    }
}

// Enums
enum Direction {
    North,
    East, 
    West,
    South
}

// then we can do syntax like 
// let my_direction = Direction::North;

// also fn can be written around enums
fn move_direction(direction: Direction) {
    // Some lame logic
}



