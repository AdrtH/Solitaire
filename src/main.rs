use rand::seq::SliceRandom;
use rand::thread_rng;
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub use crate::type::card; // TODO: faire en sorte que ca ca marche, j'en peut plus je vais canner

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

struct Board {
    deck: Stack,
    fondation: [Stack; 4],
    piles: [Stack; 7],
}

impl Board {
    fn new() -> Self {
        const STACK_NONE: Stack = Stack { stack: Vec::new() };
        Board {
            deck: create_deck(),
            fondation: {
                let mut fond: [Stack; 4] = [STACK_NONE; 4];
                for i in 0..3 {
                    fond[i] = Stack::new();
                }
                fond
            },
            piles: {
                let mut pil: [Stack; 7] = [STACK_NONE; 7];
                for i in 0..3 {
                    pil[i] = Stack::new();
                }
                pil
            },
        }
    }

    fn mov(from: &Stack, to: &Stack, size: usize) {}
}

const CARD_WIDTH: f32 = 60.0;
const CARD_HEIGHT: f32 = 87.0;

fn create_deck() -> Stack {
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

fn print_stack(mut stack: Stack) {
    while !stack.is_empty() {
        let card = stack.pop().unwrap();
        println!("{}", card.to_string());
    }
}

fn display_stack(stack: &Stack, mut d: RaylibDrawHandle, x: f32, y: f32) {
    if stack.is_empty() {
        d.draw_rectangle_rounded(
            Rectangle {
                x,
                y,
                width: 20.0,
                height: 20.0,
            },
            0.5,
            10,
            Color::DARKGREEN,
        );
    } else {
        d.draw_rectangle_rounded(
            Rectangle {
                x,
                y,
                width: 20.0,
                height: 20.0,
            },
            0.5,
            10,
            Color::WHITE,
        );
    }
}

fn display_board(board: &Board, d: RaylibDrawHandle) {
    display_stack(&board.deck, d)
}

fn main() {
    let (mut rl, thread) = raylib::init().title("Solitaire").size(900, 720).build();
    let board = Board::new();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        // d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
        display_board(&board, d);
    }
    let mut deck = create_deck();
    deck.shuffle();
    print_stack(deck);
}
