// Aufgabe 4:

// Veraendern sie den unten stehenden Code so, dass der Vector, bei der Ausgabe, die Zahlen 40, 50 und 60 enthaelt. 
// Verwenden sie dafuer einmal die "unschoene" Methode unter ausschliesslicher Verwendung der Ownership. 
// Huebschen sie den Code anschliessend mittels Borrowing auf.

fn main() {
    let mut vec0 = Vec::new();
    
    push_on_stack(&mut vec0);
    
    println!("{:?}", vec0);
}

fn push_on_stack(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(40);
    vec.push(50);
    vec.push(60);
    vec.to_owned()
}
