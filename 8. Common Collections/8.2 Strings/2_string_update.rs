fn main() {
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str() takes a string slice (&str) and appends it to the string

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1.push_str(&s2);

    println!("{}", s1);
}
