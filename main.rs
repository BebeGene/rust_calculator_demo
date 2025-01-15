// Imports standard input/output module
use std::io;

// Main function entry point
fn main() {
    // Prints welcome message
    println!("Welcome to the simple calculator project!");

    // Get the 2 numbers from user input
    let num1 = get_input("Enter first number: ");
    let num2 = get_input("Enter second number: ");

    // Prompt user to pick an operation
    println!("Choose an operation:");
    println!("1. Addition");
    println!("2. Substraction");
    println!("3. Multiplication");
    println!("4. Division");

    // Receive the choice of users input
    let choice = get_input("Enter your choice between 1 and 4: ");

    // Match statement performs the chosen operation
    match choice {
        1.0 => { println!("Result: {}", num1 + num2); }
        2.0 => { println!("Result: {}", num1 - num2); }
        3.0 => { println!("Result: {}", num1 * num2); }
        4.0 => 
            { // Handles division by 0 (not permitted)
                if num2 == 0.0 {
                    println!("Error: Can't divide by zero.");
                } else {
                    println!("Result: {}", num1 / num2);
                }
            }
            _ => println!("Invalid choice! Choose a number between 1 and 4"),
    }
}

// Function reads and parses the users input
fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();               // Initializes new string to store user input
    println!("{}", prompt);                      // Prints the prompt message
    io::stdin().read_line(&mut input).unwrap();  // Reads the user input from standard input
    input.trim().parse::<f64>().unwrap_or_else(|_| {  //Parses input as a float
            // Handles invalid input not 1-4
            println!("Invalid input! Please enter a valid #.");  
            // Retry reading the input
            get_input(prompt) 
        })
}