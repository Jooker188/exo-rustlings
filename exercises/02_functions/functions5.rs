// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// Fonction principale qui calcule et affiche le carré de 3
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// Fonction annexe qui retourne le carré d'un nombre entier
fn square(num: i32) -> i32 {
    num * num
}
