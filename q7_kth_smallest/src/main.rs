use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None; // k is out of bounds
    }
    let mut arr = arr.to_vec();
    quickselect(&mut arr, k - 1);
    Some(arr[k - 1])
}

fn quickselect(arr: &mut [i32], k: usize) {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let pivot_index = partition(arr, left, right);
        if pivot_index == k {
            break;
        } else if pivot_index < k {
            left = pivot_index + 1;
        } else {
            right = pivot_index - 1;
        }
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_index = left + (right - left) / 2;
    let pivot_value = arr[pivot_index];
    arr.swap(pivot_index, right);

    let mut i = left;
    for j in left..right {
        if arr[j] <= pivot_value {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

fn main() {

    // Prompt the user to enter the array
    println!("Enter the array of integers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a vector of integers
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Prompt the user to enter the value of k
    println!("Enter the value of k:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please enter a valid integer");

    // Convert the vector into a mutable slice for in-place modification
    let mut arr = arr.as_slice().to_owned();

    // Find the kth smallest element
    match kth_smallest(&mut arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input!"),
    }
}
