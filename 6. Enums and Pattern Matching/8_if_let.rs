fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(max) => println!("max: {}", max),
        _ => (),
    }

    if let Some(max) = some_u8_value {
        println!("max: {}", max);
    }
}
