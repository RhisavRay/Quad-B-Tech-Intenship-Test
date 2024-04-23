use std::io;

fn reverse_string(s: &str) -> String{
    s.chars().rev().collect()
}

fn main() {

    // Prompt user to enter a string
    println!("Enter a string:");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove trailing newline character
    let input = input.trim();

    // Reverse the string
    let reversed = reverse_string(input);

    // Print reversed string
    println!("The reversed string is: {}", reversed);
}