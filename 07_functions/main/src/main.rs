fn main() {
    println!("Hello, world!");
    // This is how u call another function having an argument of 100
    another_function(100);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); // This will print 4

    // Functions with Return Values
    let x = five();
    println!("The value of x is: {x}"); // This will print 5
}

fn five() -> i32 {
    5
}

//* This is how u create another function */
fn another_function(x: i8) {
    println!("Another function prints: {}", x);
}

//? N O T E S
//* `fn` is a keyword used to define function */
//* `another_function` is the name of the function having `x:i8` as a parameter */
