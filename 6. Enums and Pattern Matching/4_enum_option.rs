enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; //<- error adding i8 and Option<i8>
}
