fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {

    // Iterate over the array
    for (index, &num) in arr.iter().enumerate() {

        // If the current number is equal to the target, return its index
        if num == target {
            return Some(index);
        }
    }

    // If the target is not found, return None
    None
}

fn main() {

    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    let target = 4;

    // Call the function to find the first occurrence of the target
    match find_first_occurrence(&arr, target) {

        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}