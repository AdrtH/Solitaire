use crate::*;

pub const NB_FOND: usize = 4;
pub const NB_PILES: usize = 7;

pub struct Board {
    deck: Stack,
    playing: Stack,
    fondation: [Stack; NB_FOND],
    piles: [Stack; NB_PILES],
}

impl Board {
    pub fn new() -> Self {
        const STACK_NONE: Stack = Stack { stack: Vec::new() };
        let mut board = Board {
            deck: {
                let mut deck = create_deck();
                deck.shuffle();
                deck
            },
            playing: Stack::new(),
            fondation: {
                let mut fond: [Stack; NB_FOND] = [STACK_NONE; NB_FOND];
                for i in 0..NB_FOND {
                    fond[i] = Stack::new();
                }
                fond
            },
            piles: {
                let mut pil: [Stack; NB_PILES] = [STACK_NONE; NB_PILES];
                for i in 0..NB_PILES {
                    pil[i] = Stack::new();
                }
                pil
            },
        };
        for i in 0..NB_PILES {
            Board::mov(&mut board.deck, &mut board.piles[i], i + 1);
        }
        board
    }

    pub fn get_deck(&self) -> &Stack {
        &self.deck
    }

    pub fn get_playing(&self) -> &Stack {
        &self.playing
    }

    pub fn get_fondation(&self, i: usize) -> &Stack {
        &self.fondation[i]
    }

    pub fn get_pile(&self, i: usize) -> &Stack {
        &self.piles[i]
    }

    fn mov(from: &mut Stack, to: &mut Stack, size: usize) {
        let mut temp = Stack::new();
        for _ in 0..size {
            temp.push(from.pop().expect("Out of range"));
        }
        for _ in 0..size {
            to.push(temp.pop().expect("Out of range"));
        }
    }
}
