fn main() {
    let mut tup: (i32, f64, u8, &str, bool) = (500, 6.4, 1, "hello", true);

    tup.0 = 100;
    tup.3 = "world";

    println!("The value of tup is: {:?}", tup);

    // Destructuring
    let (x, y, z, w, b) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
