#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades"];
        let values = ["Ace", "One", "Two", "Three", "Four"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card)
            }
        }

        let deck = Deck { cards };
        return deck;
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here is the deck: {:#?}", deck)
}
