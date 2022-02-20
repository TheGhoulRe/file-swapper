use std::env;
use std::thread;
use std::fs;
use std::sync::mpsc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (tx, rx): (mpsc::Sender<(String, i8)>, mpsc::Receiver<(String, i8)>) = mpsc::channel();

    let file1: String = args[1].to_string();
    let file2: String = args[2].to_string();

    thread::spawn(move || {
        println!("Reading file 1");
        let content = fs::read_to_string(file1).unwrap();
    });

    thread::spawn(move || {
        println!("Reading file 1");
        let content = fs::read_to_string(file2).unwrap();
    });

}
