fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];

    println!("The first element is {}.", first);
    println!("The second element is {}.", second);

    // let index = 10;
    // let element = arr[index]; // error[E0277]: the len is 5 but the index is 10

    println!("arr: {:?}", arr);

    println!("Length of arr: {}", arr.len());

    let mut arr_str: [&str; 3] = ["hello", "world", "rust"];
    write_arr(&mut arr_str);
    println!("arr_str: {:?}", arr_str);
}

// fn write_arr(mut arr: [&str; 3]) {
//     arr[1] = "Cool";
//     println!("arr: {:?}", arr);
// }

fn write_arr(arr: &mut [&str; 3]) {
    arr[1] = "Cool";
    println!("arr: {:?}", arr);
}
