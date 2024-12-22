#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
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
    println!("Here is the deck: {:#?}", deck)
}
