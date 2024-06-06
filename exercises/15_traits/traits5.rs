// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    // Définit une trait SomeTrait avec une méthode par défaut some_function.
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    // Définit une trait OtherTrait avec une méthode par défaut other_function.
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

// Implémente SomeTrait pour SomeStruct.
impl SomeTrait for SomeStruct {}

// Implémente OtherTrait pour SomeStruct.
impl OtherTrait for SomeStruct {}

// Implémente SomeTrait pour OtherStruct.
impl SomeTrait for OtherStruct {}

// Implémente OtherTrait pour OtherStruct.
impl OtherTrait for OtherStruct {}

// Utilise des types associés au lieu de lier explicitement à des traits spécifiques.
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait, // Exige que le type T implémente à la fois SomeTrait et OtherTrait.
{
    // Appelle les méthodes some_function et other_function sur l'élément fourni et renvoie leur résultat logique ET.
    item.some_function() && item.other_function()
}

fn main() {
        some_func(SomeStruct {});
        some_func(OtherStruct {});
}
