// FIXME: Use an argument parsing library.

extern crate libc;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::iter::Iterator;
use std::process;
use std::vec::Vec;

#[derive(Debug)]
struct Args {
    path: String,
}

static USAGE: &'static str = "usage: hexdump [ OPTIONS ] PATH";

fn parse_args(argv: env::Args) -> Args {
    let mut path: String = String::from("");
    let mut nargs = 0;

    for arg in argv.skip(1) {
        if arg == "-h" || arg == "--help" {
            println!("{}", USAGE);
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

fn hexdump<I>(mut bytes: I, line_len: u16)
where
    I: Iterator<Item = Result<u8, Error>>,
{
    fn dig(val: u8) -> char {
        ((if val < 10 { 48 } else { 55 }) + val) as char
    }

    fn render(c: u8) -> char {
        if c < 32 || c > 126 {
            '\u{2022}'
        }
        else {
            c as char
        }
    }

    let mut pos: u64 = 0;
    let mut done = false;
    let mut line: Vec<u8> = Vec::with_capacity(line_len as usize);
    while !done {
        for line_pos in 0..line_len {
            if let Some(Ok(val)) = bytes.next() {
                // Start of line: show position.
                if line_pos == 0 {
                    print!("{:08x} | ", pos);
                }

                print!("{}{} ", dig(val >> 4), dig(val & 15));
                pos += 1;

                line.push(val);
            }
            else {
                done = true;
                break;
            }
        }

        // Pad out a partial line.
        for _ in line.len()..line_len as usize {
            print!("   ");
        }

        // Now render bytes
        print!("| ");
        for val in line.iter() {
            print!("{}", render(*val));
        }
        line.clear();

        println!("");
    }
}

const TIOCGWINSZ: libc::c_ulong = 0x40087468;

#[repr(C)]
struct winsize {
    ws_row: libc::c_ushort,    /* rows, in characters */
    ws_col: libc::c_ushort,    /* columns, in characters */
    ws_xpixel: libc::c_ushort, /* horizontal size, pixels */
    ws_ypixel: libc::c_ushort, /* vertical size, pixels */
}

fn get_winsize() -> std::io::Result<winsize> {
    let w = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    match unsafe { libc::ioctl(libc::STDOUT_FILENO, TIOCGWINSZ, &w) } {
        0 => Ok(w),
        _ => Err(std::io::Error::last_os_error()),
    }
}

fn main() -> std::io::Result<()> {
    // Rust programs ignore SIGPIPE by default.  Reenable the default
    // disposition (termination) so the program is killed if input or output
    // breaks.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let args = parse_args(env::args());
    let file = try!(File::open(args.path));

    let winsize = get_winsize().unwrap();
    let line_len = (winsize.ws_col - 14) / 4;

    hexdump(&mut BufReader::new(file).bytes(), line_len);
    Ok(())
}

