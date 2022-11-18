//extern crate whatlang;

//use whatlang::{detect, Lang, Script};

//fn main() {
//    let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de //la plej bonaj aferoj!";

//    let info = detect(text).unwrap();
//    assert_eq!(info.lang(), Lang::Epo);
//    assert_eq!(info.script(), Script::Latin);
//    assert_eq!(info.confidence(), 1.0);
//    assert!(info.is_reliable());
//}

extern crate whatlang;

use std::io;
use whatlang::detect;

fn main() {
    let mut text = String::new();
    println!("Please enter a text:");
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    if let Some(info) = detect(&text) {
        println!("Language: {}", info.lang());
        println!("Info: {:?}", info);
    } else {
        println!("Cannot recognize a language :(");
    }
}