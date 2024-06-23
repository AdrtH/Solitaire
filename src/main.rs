// AdrtH (c) 2024
use std::cmp::min;
use std::f32;

use raylib::ffi::CheckCollisionPointRec;
use raylib::ffi::DrawTexture;
use raylib::ffi::Rectangle;
use raylib::ffi::Texture;
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

fn int_to_color(x: u32) -> raylib::ffi::Color {
    raylib::ffi::Color {
        r: (x >> 24) as u8,
        g: ((x << 8) >> 24) as u8,
        b: ((x << 16) >> 24) as u8,
        a: ((x << 24) >> 24) as u8,
    }
}

fn display_card(vis_card: &VisualCard, d: &mut RaylibDrawHandle) {
    let mut card_texture: Texture;
    let tint = unsafe {
        let mouse = d.get_mouse_position();
        if CheckCollisionPointRec(mouse.into(), vis_card.collision) {
            int_to_color(0xAAAAAAFF)
        } else {
            int_to_color(0xFFFFFFFF)
        }
    };
    if vis_card.card.known {
        card_texture = match_card_texture(vis_card.card);
    } else {
        unsafe {
            card_texture = CARD_BACK.unwrap();
        }
    }
    card_texture.height = vis_card.pos.height as i32;
    card_texture.width = vis_card.pos.width as i32;
    unsafe {
        DrawTexture(
            card_texture,
            vis_card.pos.x as i32,
            vis_card.pos.y as i32,
            tint,
        );
    }
}

fn display_stack(
    stack: &Stack,
    d: &mut RaylibDrawHandle,
    x: usize,
    y: usize,
    stacked: bool,
) -> Vec<Rectangle> {
    let (card_width, card_height, hor_offset) = compute_card_dimensions(&d);
    let card_hor_offset = card_width * (1.0 - CARD_FILLING_PERC) / 2.0;
    let card_ver_offset = card_height * (1.0 - CARD_FILLING_PERC) / 2.0;
    let position = Rectangle {
        x: card_hor_offset + hor_offset + card_width * x as f32,
        y: card_ver_offset + card_height * y as f32,
        width: card_width * CARD_FILLING_PERC,
        height: card_height * CARD_FILLING_PERC,
    };
    const NO_HITBOX: Rectangle = Rectangle {
        x: -1.0,
        y: -1.0,
        width: 0.0,
        height: 0.0,
    };
    if stack.is_empty() {
        d.draw_rectangle_rounded(position, 0.2, 10, Color::DARKGREEN);
        vec![position]
    } else if !stacked {
        let card = VisualCard {
            card: stack.peek().unwrap(),
            pos: position,
            collision: position,
        };
        display_card(&card, d);
        vec![card.collision]
    } else {
        let cards = stack.as_vec();
        let stacked_card_offset = min(
            (position.y / cards.len() as f32) as i32,
            (card_height / 5.0) as i32,
        ) as f32;
        let mut pos = position;
        let mut vec = vec![];
        for i in 0..cards.len() {
            let card = VisualCard {
                card: &cards[i],
                pos,
                collision: Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: pos.width,
                    height: if i < cards.len() - 1 {
                        stacked_card_offset
                    } else {
                        pos.height
                    },
                },
            };
            display_card(&card, d);
            pos.y += stacked_card_offset;
            if card.card.known {
                vec.push(card.collision);
            } else {
                vec.push(NO_HITBOX)
            }
        }
        vec
    }
}

fn display_board(board: &mut Board, d: &mut RaylibDrawHandle) -> BoardHitboxes {
    let mut hitboxes = BoardHitboxes::new();
    hitboxes.deck = display_stack(board.get_deck(), d, 0, 0, false)[0];
    hitboxes.playing = display_stack(board.get_playing(), d, 1, 0, false)[0];
    for i in 0..NB_FOND {
        hitboxes.fondation[i] = display_stack(board.get_fondation(i), d, i + 3, 0, false)[0];
    }
    for i in 0..NB_PILES {
        hitboxes.stack[i] = display_stack(board.get_pile(i), d, i, 1, true);
    }
    return hitboxes;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Solitaire")
        .size(900, 720)
        .resizable()
        .build();
    let mut board = Board::new();
    unsafe {
        load_cards_texture();
    };
    while !rl.window_should_close() {
        rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            board = Board::new()
        }
        board.update_known();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GREEN);
        let hitbox = display_board(&mut board, &mut d);
        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            let click = hitbox.get_clicked(d.get_mouse_position());
            board.handle_click(click);
        }
    }
}
