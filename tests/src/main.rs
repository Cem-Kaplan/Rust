use std::io;

fn main() {

    println!("Bitte etwas eingeben:");

    let mut eingabe = String::new();
    
    io::stdin()
        .read_line(&mut eingabe)
        .expect("Fehler");

    println!("Ihre eingabe: {eingabe}");
}