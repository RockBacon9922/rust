// import io
use std::io;

fn main() {
    let a = 2;
    let b = 3;
    let text = "some text";
    fn add(a: i32, b: i32) -> i32 {
        // add variables then return
        return a + b;
    }
    fn justreturn(text: &str) -> &str {
        // return a string
        return text;
    }
    println!("{} + {} = {}", a, b, add(a, b));
    println!("{}", justreturn(text));
    // get user input
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    fizzbuzz()
}

fn fizzbuzz() {
    for i in 1..101 {
        let mut word = "".to_string();
        if i % 3 == 0 {
            word.push_str("Fizz");
        }
        if i % 5 == 0 {
            word.push_str("Buzz");
        }
        if word == "" {
            word = i.to_string();
        }
        println!("{}", word);
    }
}
