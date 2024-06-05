// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    // Constructeur pour créer une nouvelle instance de Package
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
       // Vérifie que le poids est supérieur ou égal à 10 grammes
        if weight_in_grams < 10 {
            // This is not how you should handle errors in Rust,
            // but we will learn about error handling later.
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            // Retourne une nouvelle instance de Package
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    // Méthode pour vérifier si l'envoi est international
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    // Méthode pour calculer les frais de transport
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // Multiplie le poids en grammes par le coût par gramme
        cents_per_gram * self.weight_in_grams
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
