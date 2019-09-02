// FIXME: Use an argument parsing library.

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::iter::Iterator;
use std::process;


#[derive(Debug)]
struct Args {
    path: String,
}


static USAGE: &'static str = "usage: hexdump [ OPTIONS ] PATH\
";

fn parse_args(argv: env::Args) -> Args {
    let mut path: String = String::from("");
    let mut nargs = 0;

    for arg in argv.skip(1) {
        if arg == "-h" || arg == "--help" {
            print!("{}", USAGE);
            process::exit(0);
        }
        else {
            // Positional arg.
            if nargs > 0 {
                writeln!(&mut std::io::stderr(), "too many arguments").unwrap();
                process::exit(2);
            }
            path = arg;
            nargs += 1;
        }
    }

    if nargs < 1 {
        writeln!(&mut std::io::stderr(), "{}", USAGE).unwrap();
        process::exit(2);
    }

    Args { path: path }
}


fn hexdump<I>(mut bytes: I, line_len: u8) 
where
    I: Iterator<Item = Result<u8, Error>>,
{
    fn dig(val: u8) -> char {
        ((if val < 10 { 48 } else { 55 }) + val) as char
    }

    let mut pos: u64 = 0;
    'outer: loop {
        for line_pos in 0 .. line_len {
            if let Some(Ok(val)) = bytes.next() {
                if line_pos == 0 {
                    print!("{:08x} | ", pos);
                }

                print!("{}{} ", dig(val >> 4), dig(val & 15));
                pos += 1;
            }
            else {
                if line_pos > 0 {
                    println!("")
                }
                break 'outer
            }
        }
        println!("")
    }
}


fn main() -> std::io::Result<()> {
    let args = parse_args(env::args());
    let file = try!(File::open(args.path));
    let buf_reader = BufReader::new(file);
    hexdump(&mut buf_reader.bytes(), 12);
    Ok(())
}

