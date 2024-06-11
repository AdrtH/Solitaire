use std::f32;

use rand::seq::SliceRandom;
use rand::thread_rng;
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub mod r#type;
use crate::r#type::card::*;

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

const NB_FOND: usize = 4;
const NB_PILES: usize = 7;

struct Board {
    deck: Stack,
    playing: Stack,
    fondation: [Stack; NB_FOND],
    piles: [Stack; NB_PILES],
}

impl Board {
    fn new() -> Self {
        const STACK_NONE: Stack = Stack { stack: Vec::new() };
        Board {
            deck: create_deck(),
            playing: Stack::new(),
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

const CARD_FILLING_PERC: f32 = 0.9;
const CARD_RATIO: f32 = 64.0 / 89.0;
const NB_COLS: f32 = 7.0;
const NB_ROW: f32 = 3.0;
const BOARD_RATIO: f32 = NB_COLS * CARD_RATIO / NB_ROW;

fn compute_card_dimensions(d: &RaylibDrawHandle) -> (f32, f32, f32) {
    let (mut width, mut height) = (d.get_screen_width() as f32, d.get_screen_height() as f32);
    let actual_ratio = width / height;
    if actual_ratio > BOARD_RATIO {
        width = height * BOARD_RATIO;
    } else if actual_ratio < BOARD_RATIO {
        height = width / BOARD_RATIO;
    }
    let card_width = width / NB_COLS;
    let card_height = height / NB_ROW;
    let hor_offset = (d.get_screen_width() as f32 - width) / 2.0;
    (card_width, card_height, hor_offset)
}

fn display_stack(stack: &Stack, d: &mut RaylibDrawHandle, x: i32, y: i32) {
    let (card_width, card_height, hor_offset) = compute_card_dimensions(&d);
    let card_hor_offset = card_width * (1.0 - CARD_FILLING_PERC) / 2.0;
    let card_ver_offset = card_height * (1.0 - CARD_FILLING_PERC) / 2.0;
    if stack.is_empty() {
        d.draw_rectangle_rounded(
            Rectangle {
                x: card_hor_offset + hor_offset + card_width * x as f32,
                y: card_ver_offset + card_height * y as f32,
                width: card_width * CARD_FILLING_PERC,
                height: card_height * CARD_FILLING_PERC,
            },
            0.5,
            10,
            Color::DARKGREEN,
        );
    } else {
        d.draw_rectangle_rounded(
            Rectangle {
                x: card_hor_offset + hor_offset + card_width * x as f32,
                y: card_ver_offset + card_height * y as f32,
                width: card_width * CARD_FILLING_PERC,
                height: card_height * CARD_FILLING_PERC,
            },
            0.5,
            10,
            Color::WHITE,
        );
    }
}

fn display_board(board: &Board, d: &mut RaylibDrawHandle) {
    display_stack(&board.deck, d, 0, 0);
    display_stack(&board.playing, d, 1, 0);
    for i in 0..NB_FOND {
        display_stack(&board.fondation[i], d, i as i32 + 3, 0);
    }
    for i in 0..NB_PILES {
        display_stack(&board.piles[i], d, i as i32, 1);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Solitaire")
        .size(900, 720)
        .resizable()
        .build();
    let board = Board::new();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        // d.draw_text("Hello, world", 12, 12, 20, Color::BLACK);
        display_board(&board, &mut d);
    }
    // let mut deck = create_deck();
    // deck.shuffle();
    // print_stack(deck);
}
