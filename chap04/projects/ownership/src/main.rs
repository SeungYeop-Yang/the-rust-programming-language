fn main() {
    // les s = "hello";                     // this is a literal
    let mut s = String::from("hello");      // this is a variable; s is valid from this point forward

    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    let r3 = &mut s;

    println!("{r3}");
}
