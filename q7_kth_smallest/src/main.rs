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
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 4;

    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input!"),
    }
}
