fn main() {
    // les s = "hello";                     // this is a literal
    let s = String::from("hello");      // this is a variable; s is valid from this point forward
    takes_ownership(s);                 // the scope of s is now over, and s is no longer valid

    let x = 5;
    makes_copy(x);

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}   // the scope of some_string is now over, and some_string is no longer valid

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
