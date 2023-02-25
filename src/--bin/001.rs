pub fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    let hello = hello();
    println!("{}", hello);
}
