// Aufgabe 2:

// In der Praesentation wurden String-Slices am Beispiel der Funktion first_word erklaert, 
// bauen sie das Beispiel nach und schreiben sie eine Funktion second_word unter Verwendung von String-Slices, 
// wenn es kein 2. Wort gibt wird nachwievor der ganze String zurueckgegeben.
use std::io;
fn main(){
    println!("Type a sentece");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read your input");

    let res = second_word(input);
    println!("{:#?}", res);
}

fn second_word(sentence: String) -> String {
    let mut sliced_string = sentence.split_whitespace();
    let _first_word = sliced_string.next();
    let res = sliced_string.next();
    println!("{:#?}", res);

    match res{
        Some(res) => res.to_owned(),
        _ => sentence
    }
}