use std::thread;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut cmd_line = std::env::args();
    cmd_line.next();

    let file1 = cmd_line.next();
    let file2 = cmd_line.next();
}
