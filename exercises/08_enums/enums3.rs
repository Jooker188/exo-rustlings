// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

enum Message {
    // Variante pour quitter
    Quit,
    // Variante pour répéter le message donné
    Echo(String),
    // Variante pour déplacer vers une nouvelle position
    Move(Point),
    // Variante pour changer la couleur avec des composants RGB
    ChangeColor(u8, u8, u8),
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    // Méthode pour changer la couleur de l'état
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    // Méthode pour marquer l'état comme étant prêt à quitter
    fn quit(&mut self) {
        self.quit = true;
    }

    // Méthode pour répéter un message dans l'état
    fn echo(&mut self, s: String) {
        self.message = s
    }

    // Méthode pour déplacer la position de l'état
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    // Méthode pour traiter les messages en fonction de leur variante
    fn process(&mut self, message: Message) {
        match message {
            // Lorsque le message est de type ChangeColor, met à jour la couleur
            Message::ChangeColor(r, g, b) => self.color = (r, g, b),
            // Lorsque le message est de type Echo, met à jour le message
            Message::Echo(s) => self.message = s,
            // Lorsque le message est de type Move, met à jour la position
            Message::Move(x) => self.position = x,
            // Lorsque le message est de type Quit, marque l'état comme prêt à quitter
            Message::Quit => self.quit = true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
