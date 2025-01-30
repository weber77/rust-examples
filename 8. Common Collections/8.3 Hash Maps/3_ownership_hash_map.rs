use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are no longer valid here, try to access them will result in an error

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
