# ct_rust

Inline Rust code directly in your Rust code.

## Example

```rust
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate inline_rust;

fn main() {
   rust!(
       fn main() {
           println!("let test = 42;");
       }
   );

   println!("{}", test) // prints 42
}
```

## How to use

Use the `rust!{..}` macro to write Rust code directly in your Rust code.
This Rust code will be compiled and executed at compile time and what it writes to stdout
will be used to replace the original macro call.
You'll need to add `#![feature(proc_macro_hygiene)]`, and use a nightly
version of the compiler that supports this feature.

### Limitations

- It is currently not possible to use variables from surrounding Rust code inside the macro.
- Choosing a rustc version is not supported, the Rust code will be compiled by the rustc found in your PATH.
This is influenced by your current default toolchain (if you are using rustup).
- The Rust code can not depend on any crates as there is no way to specify dependencies
and a single .rs cannot specify dependencies (AFAIK).
The easiest way to add this functionality would be to create a temporary cargo project
and provide special syntax to the rust! macro with configuration options for the temporary project.

## Credits
Inspired by ([@m_ou_se](https://twitter.com/m_ou_se)) who has implemented a macro to run Python code 
[inline-python](https://github.com/fusion-engineering/inline-python).
Having a macro for Rust code was the idea of the reddit user 
[u/tending](https://www.reddit.com/r/rust/comments/gwe7ut/ct_python_a_rust_macro_to_execute_python_code_at/fsv8r1l/). 
Thank you!
