use std::process;
use crate::argsdef::Args;
use clap::Parser;

mod argsdef;
mod compiler_internals;
mod getup;
mod errdef;

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
