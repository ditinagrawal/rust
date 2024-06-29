use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Guess the number : ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//? N O T E S
//* `use rand::Rng` is used to use the `Rng` trait */
//* `thread_rng` is used to get a random number generator for the current thread */
//* `gen_range(1..=100)` used to generate a random number between 1 and 100 */
//* `trim()` is used to remove any whitespace from the input */
//* `parse()` is used to convert the input into a number */
//* `std::cmp::Ordering` is a library that contains the `Ordering` enum */
//* `match` is used for pattern matching of the input */
//* `Ordering` is used to compare the input with the secret number */
