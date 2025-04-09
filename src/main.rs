fn main() {
    let greeting_message: String = get_greeting_message();
    println!("{}", greeting_message);
}

fn get_greeting_message() -> String {
    String::from("Hi bro. Welcome to the IXCodeRS Project - a dev sandbox for Rust.")
}
