extern crate whatlang;

use std::io;
use whatlang::detect;

// in the programe below, you can use 'Как Вас зовут?' as input, in Russian // language, it means what's your name to text the code.

fn main() {
    let mut text = String::new();

    println!(
        "\nGood day Mr Abimbola. This is Consesus's group assignment.\n
    Below is the list of individuals in the group:\n\n
    \t 1. Abdulyekeen Lukman \n
    \t 2. Samuel Olayinka \n
    \t 3. Ayokunle Erdwards \n
    \t 4. Gbenga Etomu \n
    \t 5. Segun \n\n"
    );

    println!("Oya, enter a text to confirm its origin:");
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
