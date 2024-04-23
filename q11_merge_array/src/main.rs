use std::io;

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

// Function to sort an array
fn sort_array(mut array: Vec<i32>) -> Vec<i32> {
    array.sort();
    array
}

// Function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    // Merge the arrays while both indices are within bounds
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged_array.push(arr1[i]);
            i += 1;
        } else {
            merged_array.push(arr2[j]);
            j += 1;
        }
    }

    // Add remaining elements from arr1
    while i < arr1.len() {
        merged_array.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2
    while j < arr2.len() {
        merged_array.push(arr2[j]);
        j += 1;
    }

    merged_array
}

fn main() {
    // Read the first array from user input
    let arr1 = read_array();
    println!("Original array 1: {:?}", arr1);

    // Read the second array from user input
    let arr2 = read_array();
    println!("Original array 2: {:?}", arr2);

    // Sort the arrays
    let sorted_arr1 = sort_array(arr1.clone());
    let sorted_arr2 = sort_array(arr2.clone());
    println!("Sorted array 1: {:?}", sorted_arr1);
    println!("Sorted array 2: {:?}", sorted_arr2);

    // Merge the sorted arrays
    let merged_array = merge_sorted_arrays(&sorted_arr1, &sorted_arr2);
    println!("Merged array: {:?}", merged_array);
}