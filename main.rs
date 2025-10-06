fn main() {
    let s = String::from("Hello There");
    println!("String's Length is: {}", find_string_length(&s))
}

// Assingment 02
fn find_string_length(s: &str) -> usize {
    s.chars().count()
}

// Assignment 01
fn fib(x: i32) -> i32 {
    if x == 1 {
        0
    } else if x == 2 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

// Assignment 03
