use std::env;

fn parse(args: Vec<String>) {
    if args.len() == 1 {
        println!("No args passed in");

        return;
    }

    for arg in args.iter().skip(1) {
        println!("{}", arg);
    }
}

fn main() {
    parse(env::args().collect());
}
