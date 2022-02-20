use std::env;
use std::thread;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file1: String = args[1].to_string();
    let file2: String = args[2].to_string();

    let mut content1 = String::new();
    let mut content2 = String::new();

    let handle1 = thread::spawn(move || {
        println!("Reading file 1");
        content1 = fs::read_to_string(file1).unwrap();
        return content1;
    });
}
