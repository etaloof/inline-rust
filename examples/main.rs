#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate inline_rust;

fn main() {
    rust!(
        fn main() {
            println!("let test = 42;");
        }
    );

    println!("{}", test)
}
