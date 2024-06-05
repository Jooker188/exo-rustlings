// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// Définit une structure Book qui contient des références à des chaînes de caractères.
struct Book<'a> {
    author: &'a str, // Référence à une chaîne de caractères pour l'auteur
    title: &'a str,  // Référence à une chaîne de caractères pour le titre
}

fn main() {
    // Crée une chaîne de caractères pour l'auteur.
    let name = String::from("Jill Smith");
    // Crée une chaîne de caractères pour le titre du livre.
    let title = String::from("Fish Flying");

    // Crée une instance de Book en utilisant des références aux chaînes de caractères créées ci-dessus.
    let book = Book { author: &name, title: &title };

    // Affiche le titre et l'auteur du livre.
    println!("{} by {}", book.title, book.author);
}
