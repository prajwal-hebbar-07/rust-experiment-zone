#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades"];
    let values = ["Ace", "One", "Two", "Three", "Four"];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
        }
    }

    let deck = Deck { cards: vec![] };
    println!("Here is the deck: {:?}", deck)
}
