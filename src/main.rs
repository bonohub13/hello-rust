use std::env;
use std::iter::FromIterator;
// use std::fs;
mod vector_test;
mod foobar;
mod fibonacci;
mod prime_number_generator;

/*
fn test_prime_number_generator(args: &Vec<String>) {
    let splitter = "-".repeat(10);
    let mut msg: String;
    let mut pn_list: Vec<u32>;
    let mut filename: String;
    let mut write_: String;

    if args[1] == "primeNumberGenerator" && args.len() == 3 {
        msg = String::from("testing prime_number_generator");
        println!("{0} {1} {0}",
                 splitter, msg);
        for i in 20..args[2].parse().unwrap() {
            println!("{0} sequence {1} {0}", splitter, i);
            pn_list = prime_number_generator::generate_prime_numbers(i);
            filename = String::from("../output/sequence")+&i.to_string()+&String::from(".txt");
            write_ = format!("{:?}", &pn_list);
            fs::write(filename, write_.as_bytes());
            println!("result {:?}", pn_list);

        }
        println!("{0}{1}{0}",
                 splitter, "-".repeat(msg.len()+2));
    }
}
*/

fn modes(args: &Vec<String>) {
    let arg_cpy = Vec::from_iter(args[1..args.len()].iter().cloned());
    let targets = ["vector", "foobar", "fibonacci", "primeNumberGenerator"];
    let splitter = "-".repeat(10);

    for arg in arg_cpy.iter() {
        if targets.iter().find(|target| target == &arg) != None {
            if arg == targets[0] {
                println!("{0} {1} {0}", splitter, arg);
                vector_test::run();
                println!("{0}{1}{0}", splitter, "-".repeat(arg.len()+2));
            } else if arg == targets[1] {
                println!("{0} {1} {0}", splitter, arg);
                foobar::foobar();
                println!("{0}{1}{0}", splitter, "-".repeat(arg.len()+2));
            } else if arg == targets[2] {
                println!("{0} {1} {0}", splitter, arg);
                let fib_list = fibonacci::generate_fibonacci();
                println!("fibonacci list:");
                println!("\tdata: {:?}",
                    fib_list);
                println!("\tlenght: {}", fib_list.len());
                println!("{0}{1}{0}", splitter, "-".repeat(arg.len()+2));
            } else if arg == targets[3] {
                println!("{0} {1} {0}", splitter, arg);
                let pn_list = prime_number_generator::generate_prime_numbers(100000);
                println!("prime numbers:");
                println!("\tdata: {:?}",
                    pn_list);
                println!("\tlenght: {}", pn_list.len());
                println!("{0}{1}{0}", splitter, "-".repeat(arg.len()+2));
            }
        } else {
            eprintln!("Error: Argument must be either of {:?}",
                targets);
            break;
        }
    }
}

fn main() {
    // collect args (argc, argv) in C/C++
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Hello, world!");
    } else {
        modes(&args);
        // test_prime_number_generator(&args);
    }
}
