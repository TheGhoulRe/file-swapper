use std::thread;
use std::io::Read;
use std::io::Write;

fn main() {
    let cmd_line = std::env::args();
    let mut args: Vec<String> = vec![];

    for arg in cmd_line {
        args.push(arg.to_string());
    }
}
