fn main() {
    let ans = find_first_a(String::from("Hi There I am Rudransh"));

    match ans {
        Option::None => println!("No A Found Inside Given String..."),
        Option::Some(val) => println!("Found character 'a' at index {}", val)
    }
}

fn find_first_a(str: String) -> Option<u32> {
    let mut index: u32 = 0;
    for c in str.chars() {
        if c == 'a' {
            return Some(index);
        }
        index = index + 1;
    }

    None
}