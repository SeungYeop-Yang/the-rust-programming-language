```
ubuntu@8643b304b1a1:/rust/chap01$ rustc --version
rustc 1.66.1 (90743e729 2023-01-10)
ubuntu@8643b304b1a1:/rust/chap01$
ubuntu@8643b304b1a1:/rust/chap01$ rustup update
info: syncing channel updates for 'stable-aarch64-unknown-linux-gnu'
info: checking for self-updates

  stable-aarch64-unknown-linux-gnu unchanged - rustc 1.66.1 (90743e729 2023-01-10)

info: cleaning up downloads & tmp directories
ubuntu@8643b304b1a1:/rust/chap01$
ubuntu@8643b304b1a1:/rust/chap01$ mkdir projects
ubuntu@8643b304b1a1:/rust/chap01$ cd projects
ubuntu@8643b304b1a1:/rust/chap01/projects$ mkdir hello_world
ubuntu@8643b304b1a1:/rust/chap01/projects$ cd hello_world/
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$ cat main.rs
fn main() {
  println!("Hello, world");
}ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$ rustc main.rs
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$ ./main
Hello, world
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$ cargo --version
cargo 1.66.1 (ad779e08b 2023-01-10)
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_world$ cd ..
ubuntu@8643b304b1a1:/rust/chap01/projects$ cargo new hello_cargo
     Created binary (application) `hello_cargo` package
ubuntu@8643b304b1a1:/rust/chap01/projects$ cd hello_cargo/
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_cargo$ cargo build
   Compiling hello_cargo v0.1.0 (/rust/chap01/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.58s
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_cargo$ ls
Cargo.lock  Cargo.toml  src  target
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_cargo$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/hello_cargo`
Hello, world!
ubuntu@8643b304b1a1:/rust/chap01/projects/hello_cargo$
```
