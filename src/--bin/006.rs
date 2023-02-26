// Reverse a string

pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for C in input.chars().rev() {
        result.push(C);
    }
    result
}

fn main() {
    let str = "hello";
    println!("{}", reverse(&str));
}
