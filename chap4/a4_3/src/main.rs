// Aufgabe 3:

// Erstellen sie ein Array mit i32 Werten, veraendern sie das Array an verschiedenen Stellen (nicht nur die erste!) unter Verwendung der folgenden Funktion:
fn main(){
    let mut arr: [i32; 5] = [1, 2, 3 ,4 ,5];
    change_slice(&mut arr[0..1], 3);
    println!("{:#?}", arr);
    change_slice(&mut arr[4..5], 4);
    println!("{:#?}", arr);

}

fn change_slice(slice:&mut [i32], value: i32) {
    slice[0] = value;
}