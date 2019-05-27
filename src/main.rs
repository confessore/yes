use std::env;
use std::ops::Add;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() > 0 {
        let mut tmp = String::new();
        for arg in args {
            tmp += &arg.add(" ");
        }
        loop {
            println!("{}", &tmp.trim());
        }
    } else {
        loop {
            println!("{}", "y".to_owned());
        }
    }
}