use crate::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Stack {
    pub stack: Vec<Card>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: Card) {
        self.stack.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&Card> {
        self.stack.last()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.stack.shuffle(&mut rng);
    }

    pub fn as_vec(&self) -> Vec<Card> {
        self.stack.to_vec()
    }
}

pub fn create_deck() -> Stack {
    let mut deck = Stack::new();
    for j in [Suit::CLUB, Suit::DIAMOND, Suit::SPADE, Suit::HEART] {
        for i in 1..14 {
            deck.push(Card {
                value: i,
                suit: j,
                known: false,
            });
        }
    }
    deck
}
