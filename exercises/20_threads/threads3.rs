// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // Crée une référence comptée atomiquement (Arc) pour partager la queue entre les threads.
    let qc = Arc::new(q);
    // Clone l'Arc pour obtenir des références supplémentaires à la queue.
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    // Clone l'émetteur (tx) pour permettre l'envoi de valeurs dans le premier thread.
    let tx1 = tx.clone();

    // Crée et lance le premier thread.
    thread::spawn(move || {
        // Parcourt la première moitié de la queue.
        for val in &qc1.first_half {
            // Affiche la valeur envoyée.
            println!("sending {:?}", val);
            // Envoie la valeur via le canal tx1.
            tx1.send(*val).unwrap();
            // Attend 1 seconde avant d'envoyer la prochaine valeur.
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Crée et lance le deuxième thread.
    thread::spawn(move || {
        // Parcourt la deuxième moitié de la queue.
        for val in &qc2.second_half {
            // Affiche la valeur envoyée.
            println!("sending {:?}", val);
            // Envoie la valeur via le canal tx.
            tx.send(*val).unwrap();
            // Attend 1 seconde avant d'envoyer la prochaine valeur.
            thread::sleep(Duration::from_secs(1));
        }
    });
}


#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}