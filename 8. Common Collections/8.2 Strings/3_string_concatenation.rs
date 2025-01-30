fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    /*
    the + operator implements the fn add(self, s: &str) -> String
    which takes ownership of self and appends the string slice s to the end of self
    */
    let s3 = s1 + &s2; // s1 is moved into s3, and s2 is not moved

    println!("{}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = s4 + "-" + &s5 + "-" + &s6;

    println!("{}", s7);

    let sx = String::from("tic");
    let s8 = format!("{}-{}-{}", sx, s5, s6);

    println!("{}", s8);
}
