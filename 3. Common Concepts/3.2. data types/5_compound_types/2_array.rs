/*
Run: rustc 2_array.rs && ./2_array
*/

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // arrays are fixed length in Rust

    println!("a[0]: {}", a[0]);

    let a = [3; 5]; // creates an array with 5 elements, all set to 3

    println!("a[0]: {}", a[0]);
}

