use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1);
    match first {
        Some(inner) => println!("Got first argument: {}", inner),
        None => panic!("Didn't get the first arguments"),
    }

    let operator = args.nth(0);
    match operator {
        Some(inner) => println!("Got operator: {}", inner),
        None => panic!("Didn't get the operator"),
    }

    let second = args.nth(0);
    match second {
        Some(inner) => println!("Got second argument: {}", inner),
        None => panic!("Didn't get the second arguments"),
    }
}
