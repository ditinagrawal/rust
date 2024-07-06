use std::io;

fn main() {
    println!("Please enter any number : ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Failed to parse number");
    let num: i8 = input.trim().parse().expect("Failed to parse number");

    // print_table(num);
    // check_prime(num);
    // factorial(num);
    // fibonnaci(num);
    // reverse_string("Hello World");
    // println!("{}", check_palindrome_string("TaT"));
    // reverse_number(number);
}

fn reverse_number(n: i32) {
    let mut m = n;
    let mut reverse_num = 0;
    while m != 0 {
        let remainder = m % 10;
        reverse_num = reverse_num * 10 + remainder;
        m /= 10;
    }
    println!("{}", reverse_num);
}

fn check_palindrome_string(s: &str) -> bool {
    let new_str: String = s.chars().rev().collect();
    s == new_str
}

fn reverse_string(s: &str) {
    let new_str: String = s.chars().rev().collect();
    println!("{}", new_str);
}

fn fibonnaci(n: i8) {
    let mut f0: i32 = 0;
    let mut f1: i32 = 1;
    println!("{}", f0);
    println!("{}", f1);
    for i in 2..n {
        let mut next: i32 = f0 + f1;
        println!("{}", next);
        f0 = f1;
        f1 = next;
    }
}

fn factorial(n: i8) {
    let mut m: i32 = n.into();
    let mut fac: i32 = 1;
    while m != 0 {
        fac *= m;
        m -= 1;
    }
    println!("{}", fac);
}

fn check_prime(n: i8) {
    let mut count = 0;
    for i in 2..n - 1 {
        if n % i == 0 {
            count = 1;
        }
    }
    if count == 1 {
        println!("Not Prime");
    } else {
        println!("Prime");
    }
}

fn print_table(n: i8) {
    for i in 1..11 {
        println!("{} * {} => {}", n, i, n * i);
    }
}
