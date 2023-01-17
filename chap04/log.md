ownership, borrowing, slices, ... how Rust lays data out in memory

Ownership is a set of rules that govern how a Rust program manages memory. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

The ownership rules:
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
fn main() {
    let s = "hello";    // s is valid from this point forward

    println!("{s}");
}                       // the scope of s is now over, and s is no longer valid
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership

ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
fn main() {
    let mut s = String::from("hello");    // s is valid from this point forward
    s.push_str(", world!");
    println!("{s}");
}                       // the scope of s is now over, and s is no longer valid
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cargo run
   Compiling ownership v0.1.0 (/rust/chap04/projects/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 1.61s
     Running `target/debug/ownership`
hello, world!
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership
```

`String` under the covers:

![string](https://doc.rust-lang.org/book/img/trpl04-01.svg)

Something called `shallow copy` in other languages, but because Rust invalidates the first variable, it's known as `move`.

Rust will never automatically create `deep copies`. `clone` is the deep copy in Rust.

```
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
fn main() {
    // les s = "hello";                     // this is a literal
    let mut s1 = String::from("hello");      // this is a variable; s is valid from this point forward
    s1.push_str(", world!");
    println!("{s1}");

    let s2 = s1;
    println!("{}", s2);

    let s3 = s2.clone();
    println!("{}, {}", s2, s3);

}                       // the scope of s is now over, and s is no longer valid
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ownership`
hello, world!
hello, world!
hello, world!, hello, world!
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$

ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
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
}ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cargo run
   Compiling ownership v0.1.0 (/rust/chap04/projects/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/ownership`
hello
5
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$
```

01/17/2023
learned about the ownership earlier, let's continue:
reference and borrowing

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

```
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
fn main() {
    // les s = "hello";                     // this is a literal
    let mut s = String::from("hello");      // this is a variable; s is valid from this point forward

    change(&mut s);

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cargo run
   Compiling ownership v0.1.0 (/rust/chap04/projects/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 1.59s
     Running `target/debug/ownership`
hello, world
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$

ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cat src/main.rs
fn main() {
    // les s = "hello";                     // this is a literal
    let mut s = String::from("hello");      // this is a variable; s is valid from this point forward

    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    let r3 = &mut s;

    println!("{r3}");
}

ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$ cargo run
   Compiling ownership v0.1.0 (/rust/chap04/projects/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 1.47s
     Running `target/debug/ownership`
hello, hello
hello
ubuntu@8643b304b1a1:/rust/chap04/projects/ownership$
```

dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
