use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();

    // Check if the array length is odd or even
    if len % 2 == 0 {
        // If the array length is even, return the average of the middle two elements
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        // If the array length is odd, return the middle element
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {

    // Prompt the user to enter the array elements
    println!("Enter the sorted array of integers (space-separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a vector of integers
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Calculate the median of the array
    let median_val = median(&arr);

    // Print the median
    println!("Median of the array: {}", median_val);
}