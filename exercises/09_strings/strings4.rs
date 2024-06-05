// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // 1. Utilisation d'une référence à une chaîne de caractères littérale
    string_slice("blue");

    // 2. Utilisation d'une chaîne de caractères convertie depuis une chaîne littérale
    string("red".to_string());

    // 3. Utilisation d'une chaîne de caractères créée à partir d'une autre chaîne de caractères
    string(String::from("hi"));

    // 4. Utilisation d'une chaîne de caractères convertie depuis une autre chaîne de caractères
    string("rust is fun!".to_owned());

    // 5. Utilisation d'une chaîne de caractères convertie depuis une autre chaîne de caractères
    string("nice weather".into());

    // 6. Utilisation d'une chaîne de caractères générée par interpolation de format
    string(format!("Interpolation {}", "Station"));

    // 7. Utilisation d'une sous-chaîne de la première lettre d'une chaîne de caractères
    string_slice(&String::from("abc")[0..1]);

    // 8. Utilisation d'une chaîne de caractères après suppression des espaces initiaux et finaux
    string_slice("  hello there ".trim());

    // 9. Utilisation d'une chaîne de caractères après remplacement d'une sous-chaîne
    string("Happy Monday!".to_string().replace("Mon", "Tues"));

    // 10. Utilisation d'une chaîne de caractères après conversion en minuscules
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
