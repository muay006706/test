use std::io;
fn main() {
    loop {let mut input = String::new();
        println!("Enter a String for Palindrome check: ");
        io::stdin().read_line(&mut input);
        if input.trim().to_lowercase() == "end" { break; }
        check_palindrome(input);
    }
    println!("Program end.");
}
//Use function Palindrome in Rust

fn check_palindrome(input: String) {
    let msg_string: String = input.trim().to_lowercase();
    let key: &str = msg_string.as_str();

    let mut Palindrome: bool = true;
    
// for

    for (index, letter) in key.chars().enumerate()  {
        if index == key.len() /2 { 
            break;
        }
        
        if letter != key.chars().rev().nth(index).unwrap() {
                Palindrome = false;
            break;
        }
    }
    if Palindrome {
        println!("\"{}\" is a palindrome:", input.trim());
    }  else {
        println!("\"{}\" is not a palindrome:", input.trim());
    }
}

