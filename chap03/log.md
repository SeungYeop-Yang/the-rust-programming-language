```
ubuntu@8643b304b1a1:/rust/chap03/projects$ cargo new variables
     Created binary (application) `variables` package
ubuntu@8643b304b1a1:/rust/chap03/projects$ cd variables/
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$ cat src/main.rs
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$ cargo run
   Compiling variables v0.1.0 (/rust/chap03/projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$ cat src/main.rs
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$ cargo run
   Compiling variables v0.1.0 (/rust/chap03/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 1.38s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
ubuntu@8643b304b1a1:/rust/chap03/projects/variables$
```
project shadowing
```
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cat src/main.rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cargo run
   Compiling shadowing v0.1.0 (/rust/chap03/projects/shadowing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.44s
     Running `target/debug/shadowing`
The value of x in the inner scope is: 12
The value of x is: 6
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cat src/main.rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cargo run
   Compiling shadowing v0.1.0 (/rust/chap03/projects/shadowing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.33s
     Running `target/debug/shadowing`
The value of x in the inner scope is: 12
The value of x is: 6
The value of spaces is: 5
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cat src/main.rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    //let spaces = spaces.len();
    spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cargo run
   Compiling shadowing v0.1.0 (/rust/chap03/projects/shadowing)
error[E0308]: mismatched types
  --> src/main.rs:15:14
   |
13 |     let spaces = "     ";
   |                  ------- expected due to this value
14 |     //let spaces = spaces.len();
15 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `shadowing` due to previous error
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cat src/main.rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cargo build
   Compiling shadowing v0.1.0 (/rust/chap03/projects/shadowing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/shadowing`
The value of x in the inner scope is: 12
The value of x is: 6
The value of spaces is: 5
The value of guess is: 42
ubuntu@8643b304b1a1:/rust/chap03/projects/shadowing$
```

project floating_type
```
ubuntu@8643b304b1a1:/rust/chap03/projects/floating_type$ cat src/main.rs
fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    println!("The value of x is {x}");
    println!("The value of y is {y}");
}
ubuntu@8643b304b1a1:/rust/chap03/projects/floating_type$ cargo fmt
ubuntu@8643b304b1a1:/rust/chap03/projects/floating_type$ cargo run
   Compiling floating_type v0.1.0 (/rust/chap03/projects/floating_type)
    Finished dev [unoptimized + debuginfo] target(s) in 1.28s
     Running `target/debug/floating_type`
The value of x is 2
The value of y is 3
ubuntu@8643b304b1a1:/rust/chap03/projects/floating_type$
```

project numeric_operations
```
ubuntu@8643b304b1a1:/rust/chap03/projects/numeric_operations$ cat src/main.rs
fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}")
}
ubuntu@8643b304b1a1:/rust/chap03/projects/numeric_operations$
ubuntu@8643b304b1a1:/rust/chap03/projects/numeric_operations$ cargo run
   Compiling numeric_operations v0.1.0 (/rust/chap03/projects/numeric_operations)
    Finished dev [unoptimized + debuginfo] target(s) in 1.46s
     Running `target/debug/numeric_operations`
sum: 15, difference: 91.2, product: 120, quotient: 1.7608695652173911, truncated: -1
ubuntu@8643b304b1a1:/rust/chap03/projects/numeric_operations$
```

project return_values
```
ubuntu@8643b304b1a1:/rust/chap03/projects/return_values$ cat src/main.rs
fn main() {
    let x = five();
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}ubuntu@8643b304b1a1:/rust/chap03/projects/return_values$ cargo run
   Compiling return_values v0.1.0 (/rust/chap03/projects/return_values)
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/return_values`
The value of x is: 5
ubuntu@8643b304b1a1:/rust/chap03/projects/return_values$
```
