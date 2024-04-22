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
    // Test cases
    let strings1 = ["flower", "flow", "flight"];
    let strings2 = ["dog", "racecar", "car"];
    
    println!("Longest common prefix of strings1: {}", longest_common_prefix(&strings1));
    println!("Longest common prefix of strings2: {}", longest_common_prefix(&strings2));
}
