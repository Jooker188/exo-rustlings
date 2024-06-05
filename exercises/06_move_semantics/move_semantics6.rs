// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Crée une chaîne de caractères mutable "Rust is great!"
    let data = "Rust is great!".to_string();

    // Passe une référence à la fonction get_char
    get_char(&data);

    // Passe la chaîne de caractères par valeur à string_uppercase, ce qui transfère la possession
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    // Retourne le dernier caractère de la chaîne
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // Convertit la chaîne en majuscules
    data = data.to_uppercase();

    // Affiche la chaîne en majuscules
    println!("{}", data);
}
