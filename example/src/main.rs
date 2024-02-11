use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle, Thread},
};

use clap::Parser;

mod algebraic_data_types;
mod power_type_system;
mod race_condition;
mod test_in_comment;

struct Slice<'a> {
    _slice: &'a mut [u8],
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn print_to_console<D: std::fmt::Display>(d: D) {
    println!("{}", d);
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _slice = Slice {
        _slice: &mut list[0..5],
    };

    let a = Arc::new(10);
    let b = a;

    std::thread::spawn(move || {
        println!("{}", b);
    });

    crate::power_type_system::main();
    crate::algebraic_data_types::main();
    crate::race_condition::main();
}
