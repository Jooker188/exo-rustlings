// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Obtient la durée écoulée depuis l'époque UNIX jusqu'à maintenant
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        // Si la durée est obtenue avec succès, affiche le nombre de secondes écoulées depuis l'époque
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        // Si une erreur se produit (par exemple, le temps du système est avant l'époque UNIX), génère une panique
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}