use std::f32;

use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub mod r#type;
use crate::r#type::board::*;
use crate::r#type::card::*;
use crate::r#type::stack::*;

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
    display_stack(board.get_deck(), d, 0, 0);
    display_stack(board.get_playing(), d, 1, 0);
    for i in 0..NB_FOND {
        display_stack(board.get_fondation(i), d, i as i32 + 3, 0);
    }
    for i in 0..NB_PILES {
        display_stack(board.get_pile(i), d, i as i32, 1);
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
