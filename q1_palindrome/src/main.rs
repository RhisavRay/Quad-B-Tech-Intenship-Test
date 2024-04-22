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
    // Test cases
    println!("{}", is_palindrome("level")); // true
    println!("{}", is_palindrome("hello")); // false
    println!("{}", is_palindrome("A man a plan a c a nalP a nam a")); // true
}
