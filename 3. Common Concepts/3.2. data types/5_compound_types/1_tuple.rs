/*
Run: rustc 1_tuple.rs && ./1_tuple
*/


fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
}

