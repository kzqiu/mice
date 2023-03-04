extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static VERSION: &'static str = "0.1.0";

fn print_usage(prog: &str, opts: Options) {
    println!("mice {}", VERSION);
    println!("");
    println!("Usage:");
    println!("{} [OPTION]... [FILE]...", prog);
    println!("");
    println!("{}", opts.usage("Catenates output to stdout."));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let mut opts = Options::new();

    opts.optflag("h", "help", "display help and exit");
    opts.optflag("V", "version", "display current version");
    opts.optflag("n", "number", "number output lines");
    opts.optopt(
        "b",
        "begin",
        "starting index of line to print (default 1)",
        "INDEX",
    );
    opts.optopt(
        "e",
        "end",
        "ending index of line to print (default EOF)",
        "INDEX",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(m) => {
            panic!("{}", m.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    if matches.opt_present("V") {
        println!("mice version: {}", VERSION);
        return;
    }

    let mut begin: i32 = 0;
    let mut end: i32 = -1;
    let number = if matches.opt_present("n") {
        true
    } else {
        false
    };

    if matches.opt_present("b") {
        begin = if let Some(bs) = matches.opt_str("b") {
            bs.parse::<i32>().unwrap_or_default()
        } else {
            panic!("");
        }
    }

    if matches.opt_present("e") {
        end = if let Some(es) = matches.opt_str("e") {
            es.parse::<i32>().unwrap_or_default()
        } else {
            panic!("");
        }
    }

    let input = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        panic!("");
    };

    print_lines(&input, number, begin, end);
}

fn print_lines(path: &str, number: bool, begin: i32, end: i32) {
    let mut i: i32 = 1;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if i == end {
                return;
            }
            if i >= begin {
                if let Ok(li) = line {
                    if number {
                        println!("{}{}  {}", " ".repeat(5 - i.ilog10() as usize), i, li);
                    } else {
                        println!("{}", li);
                    }
                }
            }
            i += 1;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
