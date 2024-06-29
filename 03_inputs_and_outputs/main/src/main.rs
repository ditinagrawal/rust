use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter your name :");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}", name);

    //* Different Formats of print */
    println!("My name is {name}", name = "Ditin");
    println!("My name is {0}, and i'm {1} years old", "Ditin", 19);
}

//? N O T E S
//* `use` is a keyword which is used to import a library */
//* `std` is a standard library which is used to import `io` library which is used to handle Inputs and Outputs */
//* `let` is a keyword which is used to declare a variable */
//* By Default Every Variables are immutable */
//* `mut` is a keyword which is used to make a variable mutable (changable) */
//* `String` is a struct which is used to represent a string */
//* `new` is a method which is used to create a new empty string */
//* `stdin()` is a method which is used to get the standard input */
//* `read_line` is a method which is used to read a line from the standard input in which we pass the reference of the variable we want to store the value */
//* `expect` is a method which is used to handle the error */
