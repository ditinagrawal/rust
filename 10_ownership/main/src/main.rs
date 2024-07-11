fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error[E0382]: borrow of moved value: `s1`
    println!("{}, world!", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{}, world!", s3);
    println!("{}, world!", s4);

    let s5 = String::from("hello");
    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);

    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("{}", s6);

    let s7 = String::from("hello");
    let s8 = no_dangle();
    println!("{}", s8);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

//? Ownership Rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

//? References and Borrowing
// Borrowing is a way to pass a reference to a value to some code without transferring ownership.
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
// The scope of the reference must be valid.

//? N O T E S
// - Rust will never automatically create “deep” copies of your data.

//? Dangling References
// Rust ensures references will never be dangling references: a pointer that references a location in memory that may have been given to someone else, by freeing the memory while preserving the pointer.

//? Summary
// - Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
// - Rust manages memory through a system of ownership with a set of rules that the compiler checks at compile time.
