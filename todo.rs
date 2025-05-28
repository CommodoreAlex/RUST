// This is a to-do list application in rust, for a ground to experiment in adapting to the language

use std::io; // Required for reading user input, standard I/O library

fn main() {

    // Using '!' we have a macro, not a function
    println!("Welcome to your To-Do App!");

    // A 'mutable' vector, tasks, of type String- empty (initialized)
    let mut tasks: Vec<String> = Vec::new();

    // An infinite loop that will run until explicitly broken with 'break'
    loop {
        println!("\nPlease choose an option: ");
        println!("1. Add a task");
        println!("2. Show tasks");
        println!("3. Exit");

        // A 'mutable' initialized string variable called choice, empty
        let mut choice = String::new();

        // Reading user input into 'choice'
        io::stdin()
        .read_line(&mut choice) // Appendign a new line, referencing value of choice with pointer
        .expect("Failed to read input!"); // Causes program to crash if an error occurs during reading

        // Case-switch equivalent of other languages
        match choice.trim() {
            "1" => { // Match arms- if equals 1 do this (=>)
                println!("Enter your task: ");
                let mut task = String::new(); // Empty string
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read task");
                tasks.push(task.trim().to_string()); // Add to vector, removing trailing whitespace, converting to string
                println!("Task added!");
            }
            "2" => {
                println!("\nYour Tasks: ");
                // .iter() for iteration over list (tasks)
                // .enumeate() gives each item an index (i) starting from 0
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task); // {} = placeholder ofr values inserted at runtime. First has i+1, Second has task
                }
            }
            "3" => {
                println!("Goodbye!");
                break; // Ending our infinite loop (while true)
            }
            _ => println!("Invalid option, please try again."), // This is else (_=>) with a match arm, to catch everything else
        }
    }
}
