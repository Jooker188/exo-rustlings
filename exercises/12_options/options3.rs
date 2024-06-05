// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // Correspond à une référence à `y` avec `&y`.
    match &y {
        // Si `y` contient une valeur, extraction dans `p` et affiche les coordonnées.
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        // Si `y` est `None`, ça génère une panique.
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
