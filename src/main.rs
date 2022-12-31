//! A simple, fast, no-dependencies UNIX utility to print the contents of stdin to the terminal
//! *and* forward them to stdout at the same time. Useful for debugging.

use std::{
    collections::LinkedList,
    io::{stdin, stdout, Read, Write},
    process::exit,
};

const USAGE: &str = "USAGE: foo | pt [options] | bar

pt: a simple, fast, no-dependencies UNIX utility to print the contents of stdin
to the terminal *and* forward them to stdout at the same time.

Options:

--help -h -?            Show this help message.

--buffer-size -b SIZE   Specify the buffer size in bytes. Using a larger buffer 
                        size can improve throughput but also may increase 
                        latency for individual batches of input, such as lines.
                        Default: 2097152 (i.e., 2MB)";

const BUF_SIZE_DEFAULT: usize = 2 << 20; // 2MB

#[derive(Debug, Clone, Copy)]
struct Options {
    buf_size: usize,
    help: bool,
}

impl Options {
    pub fn parse() -> Self {
        let mut options = Options {
            buf_size: BUF_SIZE_DEFAULT,
            help: false,
        };

        let mut args: LinkedList<String> = std::env::args().skip(1).collect();

        while !args.is_empty() {
            let next = args.pop_front().unwrap();

            match next.as_str() {
                "--help" | "-h" | "-?" => options.help = true,
                "--buffer-size" | "-b" => {
                    let Some(size_str) = args.pop_front() else {
                        exit_with_error(1, &format!("--buffer-size (-b) requires a value.\n\n{USAGE}"))
                    };
                    match size_str.parse::<usize>() {
                        Ok(size) => options.buf_size = size,
                        Err(err) => exit_with_error(
                            1,
                            &format!(
                                "--buffer-size (-b) value must be an intger: {err}\n\n{USAGE}"
                            ),
                        ),
                    }
                }
                unknown => exit_with_error(1, &format!("Unrecognized flag: {unknown}\n\n{USAGE}")),
            }
        }

        options
    }
}

fn main() {
    let options = Options::parse();

    if options.help {
        eprintln!("{USAGE}");
        exit(0);
    }

    let Ok(mut tty) = std::fs::OpenOptions::new().write(true).open("/dev/tty") else {
        exit_with_error(2, "Unable to open /dev/tty.");
    };
    let mut locked_stdin = stdin().lock();
    let mut locked_stdout = stdout().lock();

    let mut buf = vec![0u8; options.buf_size];
    let buf = buf.as_mut_slice();

    loop {
        let bytes = match locked_stdin.read(buf) {
            Ok(0) => break, // EOF
            Ok(bytes) => bytes,
            Err(err) => {
                exit_with_error(3, &format!("I/O Error reading from stdin: {err}"));
            }
        };

        if let Err(err) = locked_stdout.write_all(&buf[..bytes]) {
            exit_with_error(3, &format!("I/O Error writing to stdout: {err}"));
        }

        if let Err(err) = tty.write_all(&buf[..bytes]) {
            exit_with_error(4, &format!("I/O Error writing to tty: {err}"));
        }
    }
}

fn exit_with_error(ecode: i32, msg: &str) -> ! {
    eprintln!("{msg}");
    exit(ecode);
}
