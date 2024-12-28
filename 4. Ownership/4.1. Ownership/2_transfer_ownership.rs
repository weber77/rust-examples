fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1); // error because s1 is moved to s2 and s1 is dropped out of scope
}
