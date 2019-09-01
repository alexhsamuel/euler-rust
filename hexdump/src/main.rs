// FIXME: Use an argument parsing library.

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
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


fn hexdump(reader: &mut BufReader<File>) {
    let mut buf: [u8; 8] = [0; 8];
    loop {
        let num_read = reader.read(&mut buf).unwrap();
        for i in 0 .. num_read {
            print!("{:02x} ", buf[i]);
        }
        println!("");
        if num_read < 8 {
            break
        }
    }
}


fn main() -> std::io::Result<()> {
    let args = parse_args(env::args());
    let file = try!(File::open(args.path));
    let mut buf_reader = BufReader::new(file);
    hexdump(&mut buf_reader);
    Ok(())
}

