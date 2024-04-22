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

    // Test cases
    println!("Is 17 prime? {}", is_prime(17)); // true
    println!("Is 20 prime? {}", is_prime(20)); // false
    println!("Is 0 prime? {}", is_prime(0)); // false
    println!("Is 1 prime? {}", is_prime(1)); // false
}