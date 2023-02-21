use std::str::FromStr; // Importing the FromStr Trait from the Standard Library
use std::env; // Importing the Environment Module from the Standard Library

// ! Main Function
fn main() {
    let mut numbers = Vec::new(); // Creating a vector of numbers to store the command-line arguments (mutable), type u32

    for arg in env::args().skip(1) {
        // Iterating over the command-line arguments, skipping the first one (the program name)
        numbers.push(
            // Pushing the command-line arguments to the vector
            // Parsing the command-line arguments to u32
            u32::from_str(&arg).expect("error parsing argument")
        ); // Handling the error if the argument is not a number
    }

    if numbers.len() == 0 {
        // If the vector is empty
        eprintln!("Usage: gcd NUMBER..."); // Print the usage message to the standard error stream
        std::process::exit(1); // Exit the program with a non-zero exit code
    }

    let mut d = numbers[0]; // Setting the first number as the greatest common divisor
    for m in &numbers[1..] {
        // Iterating over the vector, starting from the second number
        d = gcd(d, *m); // Calling the gcd function to find the greatest common divisor
        //_ *m is a dereference operator, it gets the value of the reference &m
        // ? Def: A reference is a pointer to a value
        // ? Def: A dereference operator is an operator that gets the value of a pointer
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d); // Printing the result to the standard output stream
    // {:?} is a placeholder for the vector of numbers, {} is a placeholder for the greatest common divisor
}

// ! GCD Function
fn gcd(mut n: u32, mut m: u32) -> u32 {
    // Defining the gcd function, takes two numbers and returns the greatest common divisor
    assert!(n != 0 && m != 0); // Asserting that the numbers are not zero
    // * Note: In Rust, assertions check if the condition is true, if it is not, the program panics (exits) with a message
    while m != 0 {
        // While the second number is not zero
        if m < n {
            // If the second number is less than the first number
            let t = m; // Setting the second number as the temporary variable
            // * Note: In Rust, variables are immutable by default, at the end of the scope, the variable is dropped
            m = n; // Setting the first number as the second number
            n = t; // Setting the temporary variable as the first number
        }
        m = m % n; // Setting the remainder of the second number divided by the first number as the second number
    }
    n // Returning n; in Rust, the last expression is returned if there is no return statement (implicit return) nor ; (semicolon)
}

// ! Testing the GCD Function
#[test] // Defining a test module for the gcd function
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1); // Asserting that the greatest common divisor of 14 and 15 is 1
    assert_eq!(gcd(15, 15), 15); // Asserting that the greatest common divisor of 15 and 15 is 15
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11); // Asserting that the greatest common divisor of 2 * 3 * 5 * 11 * 17 and 3 * 7 * 11 * 13 * 19 is 3 * 11
}

// * Note: In Rust, tests are defined in the same file as the code they test
// * Note: To run the tests, use the command: cargo test