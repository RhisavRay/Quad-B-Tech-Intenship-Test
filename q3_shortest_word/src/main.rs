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

    // Test cases
    println!("{:?}", shortest_word("apple banana cherry")); // Some("apple")
    println!("{:?}", shortest_word("banana cherry")); // Some("banana")
    println!("{:?}", shortest_word("")); // None
}