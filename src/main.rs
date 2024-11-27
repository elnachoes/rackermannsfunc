#![recursion_limit = "9223372036854775807"]

use clap::Parser;
use std::time::Instant;

fn ackermanns_func(x: i32, y: i32) -> i32 {
    if x == 0 {
        y + 1
    } else if y == 0 {
        ackermanns_func(x - 1, 1)
    } else {
        ackermanns_func(x - 1, ackermanns_func(x, y - 1))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = 0)]
    pub x: i32,

    #[arg(short, default_value_t = 0)]
    pub y: i32,
}

fn main() {
    let args = Args::parse();
    println!("processing, this may take multiple lifetimes of the universe...");
    let start_time = Instant::now();
    let result = ackermanns_func(args.x, args.y);
    let elapsed_time = start_time.elapsed();
    println!(
        "{}, seconds elapsed : {}",
        result,
        elapsed_time.as_secs_f64()
    )
}
