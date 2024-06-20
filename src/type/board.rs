use crate::*;

pub const NB_FOND: usize = 4;
pub const NB_PILES: usize = 7;

#[derive(PartialEq)]
pub enum StackType {
    DECK,
    PLAYING,
    FONDATION,
    PILES,
    NONE,
}

pub struct Click {
    stack_type: StackType,
    index: usize,
    card: usize,
}

pub struct Board {
    deck: Stack,
    playing: Stack,
    fondation: [Stack; NB_FOND],
    piles: [Stack; NB_PILES],
    click: Click,
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
            click: Click {
                stack_type: StackType::NONE,
                index: 0,
                card: 0,
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

    pub fn get_playing(&mut self) -> &mut Stack {
        &mut self.playing
    }

    pub fn get_fondation(&mut self, i: usize) -> &mut Stack {
        &mut self.fondation[i]
    }

    pub fn get_pile(&mut self, i: usize) -> &mut Stack {
        &mut self.piles[i]
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

    pub fn update_known(&mut self) {
        let playing = self.get_playing().pop();
        if playing.is_some() {
            let mut card = playing.unwrap();
            card.known = true;
            self.get_playing().push(card);
        }
        for i in 0..NB_PILES {
            let card_opt = self.get_pile(i).pop();
            if card_opt.is_none() {
                continue;
            }
            let mut card = card_opt.unwrap();
            card.known = true;
            self.get_pile(i).push(card);
        }
    }

    fn refill(&mut self) {
        self.deck = self.playing.clone().map(|c| Card {
            value: c.value,
            suit: c.suit,
            known: !c.known,
        });
        self.deck.revert();
        self.playing = Stack::new();
    }

    pub fn handle_click(&mut self, click: Click) {
        if click.stack_type == StackType::DECK {
            if self.deck.length() > 0 {
                Board::mov(&mut self.deck, &mut self.playing, 1);
            } else {
                self.refill();
            }
        }
    }
}

pub struct BoardHitboxes {
    pub deck: Rectangle,
    pub playing: Rectangle,
    pub fondation: [Rectangle; NB_FOND],
    pub stack: [Vec<Rectangle>; NB_PILES],
}

impl BoardHitboxes {
    pub fn new() -> Self {
        BoardHitboxes {
            deck: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            playing: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            fondation: [
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
            ],
            stack: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
        }
    }

    pub fn get_clicked(self, pos: Vector2) -> Click {
        unsafe {
            if CheckCollisionPointRec(pos.into(), self.deck) {
                return Click {
                    stack_type: StackType::DECK,
                    index: 0,
                    card: 0,
                };
            } else if CheckCollisionPointRec(pos.into(), self.playing) {
                return Click {
                    stack_type: StackType::PLAYING,
                    index: 0,
                    card: 0,
                };
            }
            for i in 0..NB_FOND {
                if CheckCollisionPointRec(pos.into(), self.fondation[i]) {
                    return Click {
                        stack_type: StackType::FONDATION,
                        index: i,
                        card: 0,
                    };
                }
            }
            for i in 0..NB_PILES {
                let current_vec = self.stack[i].clone();
                for j in 0..current_vec.len() {
                    if CheckCollisionPointRec(pos.into(), current_vec[j]) {
                        return Click {
                            stack_type: StackType::PILES,
                            index: i,
                            card: j,
                        };
                    }
                }
            }
        }
        Click {
            stack_type: StackType::NONE,
            index: 0,
            card: 0,
        }
    }
}
