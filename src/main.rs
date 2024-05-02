use rand::seq::SliceRandom;
use rand::thread_rng;
use raylib::prelude::*;

#[derive(Clone, Copy)]
enum Suit {
    CLUB,
    DIAMOND,
    SPADE,
    HEART,
}

struct Card {
    value: i16,
    suit: Suit,
}

impl Card {
    fn to_string(self) -> String {
        let s = match self.suit {
            Suit::HEART => "Heart",
            Suit::SPADE => "Spade",
            Suit::CLUB => "Club",
            Suit::DIAMOND => "Diamond",
        };
        let string_val = self.value.to_string();
        let val = match self.value {
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => &string_val,
        };
        format!("{},{}", s, val)
    }
}

struct Stack {
    stack: Vec<Card>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn push(&mut self, item: Card) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&Card> {
        self.stack.last()
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.stack.shuffle(&mut rng);
    }
}

fn create_deck() -> Stack {
    let mut deck = Stack::new();
    for j in [Suit::CLUB, Suit::DIAMOND, Suit::SPADE, Suit::HEART] {
        for i in 1..14 {
            deck.push(Card { value: i, suit: j });
        }
    }
    return deck;
}

fn print_stack(mut stack: Stack) {
    while !stack.is_empty() {
        let card = stack.pop().unwrap();
        println!("{}", card.to_string());
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().title("Solitaire").size(900, 720).build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        // d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
    }
    let mut deck = create_deck();
    deck.shuffle();
    print_stack(deck);
}
