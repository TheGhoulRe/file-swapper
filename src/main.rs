use std::env;
use std::thread;
use std::fs;
use std::sync::mpsc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (tx, rx): (mpsc::Sender<(String, i8)>, mpsc::Receiver<(String, i8)>) = mpsc::channel();
    let tx1 = tx.clone();

    let file1: String = args[1].to_string();
    let file2: String = args[2].to_string();
    let newFile2 = file1.clone();
    let newFile1 = file2.clone();

    thread::spawn(move || {
        println!("Reading file 1");
        let content = fs::read_to_string(file1).unwrap();
        tx.send((content, 1)).unwrap();
    });
    
    thread::spawn(move || {
        println!("Reading file 2");
        let content = fs::read_to_string(file2).unwrap();
        tx1.send((content, 2)).unwrap();
    });

    let mut contents: [String;2] = ["".to_string(), "".to_string()];

    for received in rx {
        contents[received.1 as usize] = received.0;
    }
    let [content1, content2] = contents;
}
