use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file1 = &args[1];
    let file2 = &args[2];

    let mut content1 = String::new();
    let mut content2 = String::new();

}
