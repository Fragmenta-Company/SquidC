use crate::argsdef::Args;
use clap::Parser;
use std::{process, thread};
use std::sync::Arc;

mod argsdef;
mod compiler_internals;
mod errdef;
mod getup;

#[macro_use]
mod macrodefs;
mod targetdef;

const VM_NAMING_CONVENTION: &str = "Squid Compiler";

/// Contains tools for checking updates, getting current version and others.
#[cfg(not(test))]
fn version_args(args: &Args) {
    if args.check_updates {
        println!("Current version: {}", env!("CARGO_PKG_VERSION"));

        getup::get_update().iter().rev().for_each(move |string| {
            println!("{string}");
        });

        process::exit(0);
    }

    use crate::targetdef::TARGET;

    if args.version {
        dev_print!("---- SVDK ---- ---- SVDK ---- SVDK ---- ---- SVDK ----");
        println!(
            "{} {} for {}",
            VM_NAMING_CONVENTION,
            env!("CARGO_PKG_VERSION"),
            TARGET
        );
        dev_print!("---- SVDK ---- ---- SVDK ---- SVDK ---- ---- SVDK ----");
        process::exit(0);
    }
}

#[cfg(not(test))]
fn main() {
    compiler_internals::tokenizer::test();

    let args = Args::parse();

    version_args(&args);

    let optimizations = args.optimizations;

    if let Some(o) = optimizations {
        match o {
            0 => {}
            1 => {}
            2 => {}
            3 => {}
            _ => {}
        }
    }
}

fn _unused() {

    // Assume we have an Arc
    let arc = Arc::new(String::from("Hello"));

    let arc_clone = Arc::clone(&arc);
    let arc_clone2 = Arc::clone(&arc);

    // Spawn a new thread
    let child = thread::spawn(move || {

        // Get a raw pointer to the data inside the Arc
        let ptr = Arc::into_raw(arc_clone);

        // UNSAFE: Cast the raw pointer to a mutable reference and modify the String
        unsafe {
            let string_ref: &mut String = &mut *(ptr as *mut String);
            string_ref.push_str(", World!");
            println!("{}", string_ref);
        }
    });

    // Spawn a new thread
    let child2 = thread::spawn(move || {
        // Get a raw pointer to the data inside the Arc
        let ptr = Arc::into_raw(arc_clone2);


        // UNSAFE: Cast the raw pointer to a mutable reference and modify the String
        unsafe {
            let string_ref: &mut String = &mut *(ptr as *mut String);
            string_ref.push_str(", worldo!");
            println!("{}", string_ref);
        }
    });

    child.join().unwrap();

    child2.join().unwrap();


    process::exit(0);

}
