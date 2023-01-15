```
ubuntu@8643b304b1a1:/rust$ cd chap02/projects/
ubuntu@8643b304b1a1:/rust/chap02/projects$ ls
ubuntu@8643b304b1a1:/rust/chap02/projects$ cargo new guessing_game
     Created binary (application) `guessing_game` package
ubuntu@8643b304b1a1:/rust/chap02/projects$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Hello, world!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

    io::stdin()
        .read_line(&mut guess);
        //.expect("Failed to read line");

    println!("You guessed: {guess}");
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
warning: unused `Result` that must be used
  --> src/main.rs:9:5
   |
9  | /     io::stdin()
10 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
^C
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
1
You guessed: 1

ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Updating crates.io index
  Downloaded getrandom v0.2.8
  Downloaded ppv-lite86 v0.2.17
  Downloaded rand v0.8.5
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded cfg-if v1.0.0
  Downloaded libc v0.2.139
  Downloaded 7 crates (824.7 KB) in 1.00s
   Compiling libc v0.2.139
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 22.60s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
1
You guessed: 1

ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 77
Please input your guess.
1
You guessed: 1

ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You wing!"),
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo build
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:21:21
   |
21 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: associated function defined here

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo build
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.68s
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
55
You guessed: 55
Too small!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 77
Please input your guess.
78
You guessed: 78
Too big!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 98
Please input your guess.
98
You guessed: 98
You win!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 89
Please input your guess.
my guess
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:19:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 2
Please input your guess.
1
You guessed: 1
Too small!
3
You guessed: 3
Too big!
2
You guessed: 2
You win!
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:20:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.62s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 80
Please input your guess.
79
You guessed: 79
Too small!
81
You guessed: 81
Too big!
80
You guessed: 80
You win!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.62s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 74
Please input your guess.
75
You guessed: 75
Too big!
72
You guessed: 72
Too small!
74
You guessed: 74
You win!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cat src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // create a mutable variable guess of an empty string (of String type)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$ cargo run
   Compiling guessing_game v0.1.0 (/rust/chap02/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.62s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
50
You guessed: 50
Too small!
75
You guessed: 75
Too small!
87
You guessed: 87
Too small!
93
You guessed: 93
You win!
ubuntu@8643b304b1a1:/rust/chap02/projects/guessing_game$
```
