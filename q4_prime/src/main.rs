use std::io;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    // Check for divisibility by numbers from 2 to sqrt(n)
    for i in 2..=f64::sqrt(n as f64) as u64 {
        if n % i == 0 {
            return false; // Found a divisor, so n is not prime
        }
    }

    true // No divisors found, so n is prime
}

fn main() {

    // Prompt the user to enter a number
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into an unsigned 64-bit integer
    let input: u64 = input.trim().parse().expect("Please enter a valid number");

    // Check if the input number is prime
    if is_prime(input) {
        println!("{} is a prime number", input);
    } else {
        println!("{} is not a prime number", input);
    }
}