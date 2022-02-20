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
    let new_file2 = file1.clone();
    let new_file1 = file2.clone();

    thread::spawn(move || {
        println!("Reading file 1");
        let content = fs::read_to_string(file1).expect("there was an error in file 1");
        tx.send((content, 0)).unwrap();
    });
    
    thread::spawn(move || {
        println!("Reading file 2");
        let content = fs::read_to_string(file2).expect("there was an error in file 2");
        tx1.send((content, 1)).unwrap();
    });

    let mut contents: [String;2] = ["".to_string(), "".to_string()];

    for received in rx {
        contents[received.1 as usize] = received.0;
    }
    let [content1, content2] = contents;

    let handler1 = thread::spawn(move || {
        println!("Writing to new file 1");
        fs::write(new_file1, content1).expect("cannot write file");
        println!("File 1 written");
    });

    let handler2 = thread::spawn(move || {
        println!("Writing to new file 1");
        fs::write(new_file2, content2).expect("cannot write file");
        println!("File 1 written");
    });

    handler1.join().expect("handler 1 had an error");
    handler2.join().expect("handler 2 had an error");
}
