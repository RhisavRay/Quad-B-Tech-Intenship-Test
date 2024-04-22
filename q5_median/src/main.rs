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

    // Test cases
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("Median of arr1: {}", median(&arr1)); // 3.0
    println!("Median of arr2: {}", median(&arr2)); // 3.5
}
