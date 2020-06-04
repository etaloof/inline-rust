#![feature(proc_macro_span)]
//! This crate provides a proc macro which takes some rust code and compiles it at compile time.
//! Whatever this code writes to stdout will be returned by this macro.
//! If the debug flag is set the output (stdout and stderr) of rustc and the given rust code
//! will be prepended as a doc comment for inspection with cargo expand.


use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use std::process::Command;
use tempdir::TempDir;
use std::path::Path;

fn execute_command(mut command: Command, identifier: &str) -> Result<(String, TokenStream), TokenStream> {
    let output = command.output();
    let output = match output {
        Ok(output) if output.stderr.is_empty() => output,
        Ok(output) => {
            let err = std::str::from_utf8(&output.stderr).unwrap();
            return Err(quote!(compile_error!(#err)));
        }
        Err(err) => {
            let err = err.to_string();
            return Err(quote!(compile_error!(#err)));
        }
    };
    let output_stdout = std::str::from_utf8(&output.stdout).unwrap();
    let output_stderr = std::str::from_utf8(&output.stderr).unwrap();

    let stdout = format!("{} stdout", identifier);
    let stderr = format!("{} stderr", identifier);

    let tokens = quote!(
        #[doc = #stdout]
        #[doc = #output_stdout]
        #[doc = #stderr]
        #[doc = #output_stderr]
    );

    let out = String::from_utf8(output.stdout).unwrap();
    Ok((out, tokens))
}

fn execute_code(input: &Path) -> Result<(String,TokenStream), TokenStream> {
    let run_command = Command::new(&input);
    execute_command(run_command, "code")
}

// Invoke rustc to compile the input file and write the object file to output
fn compile_file(input: &Path, output: &Path) -> Result<TokenStream, TokenStream> {
    let mut command = Command::new("rustc");
    command.arg("-o")
        .arg(&output)
        .arg(&input);
    let (_, result) = execute_command(command, "rustc")?;
    Ok(result)
}

fn ct_rust(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let temporary_directory = TempDir::new("ct_rust").unwrap();

    let temporary_path = if cfg!(feature = "debug") {
        temporary_directory.into_path()
    } else {
        temporary_directory.path().to_owned()
    };

    let program_source_path = temporary_path.join("ct_rust_impl.rs");
    let program_output_path = temporary_path.join("ct_rust_impl.o");

    let rust_source = input.to_string();
    std::fs::write(&program_source_path, &rust_source).unwrap();

    let compile = compile_file(&program_source_path, &program_output_path)?;
    let (output, exec) = execute_code(&program_output_path)?;

    let tokens: TokenStream = syn::parse_str(&output)
        .map_err(|err| {
            let err = err.to_string();
            quote!(compile_error!(#err))
        })?;

    Ok(
        if cfg!(feature = "debug") {
            quote!(
                #[doc = #rust_source]
                #compile
                #exec
                #tokens
            )
        } else {
            quote!(
                #tokens
            )
        }
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn rust(input: TokenStream1) -> TokenStream1 {
    TokenStream1::from(match ct_rust(proc_macro2::TokenStream::from(input)) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    })
}
