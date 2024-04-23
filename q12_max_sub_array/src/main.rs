use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {

    let mut max_sum = i32::MIN;
    let mut current_sum = 0;

    for &num in arr {
        current_sum += num;
        max_sum = std::cmp::max(max_sum, current_sum);
        current_sum = std::cmp::max(current_sum, 0);
    }

    max_sum
}

// Function to read an array from user input
fn read_array() -> Vec<i32> {
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input into a vector of integers
    input.trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect()
}

fn main() {

    // Read the array from user input
    let nums = read_array();

    // Find the maximum subarray sum
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}