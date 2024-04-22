use std::io;

fn shortest_word(s: &str) -> Option<&str> {

    // Split the string into words
    let words: Vec<&str> = s.split_whitespace().collect();

    // Check if there are any words in the string
    if words.is_empty() {
        return None;
    }

    // Find the shortest word
    let shortest = words.iter().min_by_key(|&word| word.len());

    // Return the shortest word, if found
    shortest.copied()
}

fn main() {

    // Prompt the user to enter a string
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim leading and trailing whitespace from the input
    let input = input.trim();

    // Call the shortest_word function with the user input
    match shortest_word(&input) {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("The input string is empty"),
    }
}