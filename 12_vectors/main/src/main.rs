fn main() {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = Vec::<i32>::new();

    v.push(5);
    v.push(6);
    v.push(7);

    println!("v: {:?}", v);

    let mut w = vec![1, 2, 3];
    w.pop();
    println!("w: {:?}", w);
}
