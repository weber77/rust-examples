fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v2 {
        *i += 50;
    }
}
