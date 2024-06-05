// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// Une structure générique qui enveloppe une valeur de type T.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // Méthode associée pour créer une nouvelle instance de Wrapper<T> avec une valeur spécifiée.
    pub fn new(value: T) -> Self {
        // Retourne une nouvelle instance de Wrapper<T> avec la valeur spécifiée.
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
