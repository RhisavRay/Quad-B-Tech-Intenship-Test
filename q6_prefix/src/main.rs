use std::io;

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new(); // If the array is empty, return an empty string
    }

    // Iterate over the characters of the first string
    for (i, c) in strings[0].chars().enumerate() {
        // Check if the character is present in all other strings
        if !strings.iter().all(|s| s.chars().nth(i) == Some(c)) {
            // If not, return the prefix up to the current character
            return strings[0][..i].to_string();
        }
    }

    // If all characters match, return the first string as the common prefix
    strings[0].to_string()
}

fn main() {

    // Prompt the user to enter the words
    println!("Enter the words separated by spaces:");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input string into words
    let words: Vec<&str> = input.trim().split_whitespace().collect();

    // Calculate the longest common prefix
    let prefix = longest_common_prefix(&words);

    // Print the longest common prefix
    println!("Longest common prefix: {}", prefix);
}
