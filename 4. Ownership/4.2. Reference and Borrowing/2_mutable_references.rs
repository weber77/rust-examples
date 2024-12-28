fn main() {
    let mut s1 = String::from("hello");
    println!("The value of s1: '{}'", s1);

    change(&mut s1);
    println!("The new value of s1: '{}'", s1);

    /*
       let r1 = &mut s1;
       // fails, you can't have multiple reference to a mutable object.
       // you must use r1 before you can make another reference to s1
       let r2 = &mut s1;
    */
}

fn change(s: &mut String) {
    s.push_str(", world");
}
