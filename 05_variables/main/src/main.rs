fn main() {
    let x = 5;
    //? x = 8;  ------> Gives Error
    println!("The value of x is: {}", x);

    let mut y = 5;
    //? y = 10   ------> No Error, Gives Warning
    println!("The value of y is: {}", y);

    const three_hours_in_seconds: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", three_hours_in_seconds);

    //? Shadowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}

//? N O T E S
//* `let` is used to bind a value to a variable */
//* `mut` is used to make a variable mutable */
//* `Shadowing` is a feature of Rust that allows you to use the same name for different purposes */
//* `const` is used to declare a constant. It can never be chnaged even not with `mut` */
