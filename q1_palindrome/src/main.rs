use std::io;

fn is_palindrome(s: &str) -> bool {
    // Convert the string to lowercase for case-insensitive comparison
    let s = s.to_lowercase();

    // Get the length of the string
    let len = s.len();

    // Convert the string to a sequence of characters
    let chars: Vec<char> = s.chars().collect();

    // Iterate over the characters using a for loop
    for i in 0..len / 2 {
        // Compare characters at corresponding positions from both ends
        if chars[i] != chars[len - 1 - i] {
            return false; // If characters are not equal, it's not a palindrome
        }
    }

    // If the loop completes without returning false, the string is a palindrome
    true
}

fn main() {
    // Get input from the user
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove trailing newline character
    let input = input.trim();

    // Test if the input string is a palindrome
    if is_palindrome(input) {
        println!("The input string \"{}\" is a palindrome", input);
    } else {
        println!("The input string \"{}\" is not a palindrome", input);
    }
}
