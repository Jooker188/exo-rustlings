// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

#[test]
fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    // Crée une instance de Planet représentant Saturne, partageant la référence au soleil.
    let saturn = Planet::Saturn(Rc::clone(&sun));
    // Affiche le nombre de références fortes au soleil après la création de Saturne.
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 références
    // Affiche les détails de Saturne.
    saturn.details();

    // Crée une instance de Planet représentant Uranus, partageant la référence au soleil.
    let uranus = Planet::Uranus(Rc::clone(&sun));
    // Affiche le nombre de références fortes au soleil après la création d'Uranus.
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 références
    // Affiche les détails d'Uranus.
    uranus.details();

    // Crée une instance de Planet représentant Neptune, partageant la référence au soleil.
    let neptune = Planet::Neptune(Rc::clone(&sun));
    // Affiche le nombre de références fortes au soleil après la création de Neptune.
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 références
    // Affiche les détails de Neptune.
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    // Supprime la référence à la Terre.
    drop(earth);
    // Affiche le nombre de références fortes restantes au soleil après la suppression de la Terre.
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 références

   // Supprime la référence à Vénus.
   drop(venus);
   // Affiche le nombre de références fortes restantes au soleil après la suppression de Vénus.
   println!("reference count = {}", Rc::strong_count(&sun)); // 2 références

   // Supprime la dernière référence à Mercure.
   drop(mercury);
   // Affiche le nombre de références fortes restantes au soleil après la suppression de Mercure.
   println!("reference count = {}", Rc::strong_count(&sun)); // 1 référence
   assert_eq!(Rc::strong_count(&sun), 1);
}
