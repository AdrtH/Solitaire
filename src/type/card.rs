use raylib::ffi::LoadTexture;
use raylib::ffi::Texture2D;
use std::os::raw::c_char;

pub static mut CARD_BACK: Option<Texture2D> = None;

pub unsafe fn load_cards_texture() {
    CARD_BACK = Some(LoadTexture(
        "cards/svg_playing_cards/backs/png_96_dpi/red2.png\0".as_ptr() as *const c_char,
    ));
}

#[derive(Clone, Copy)]
pub enum Suit {
    CLUB,
    DIAMOND,
    SPADE,
    HEART,
}

pub struct Card {
    pub value: i16,
    pub suit: Suit,
    pub known: bool,
}

impl Card {
    pub fn to_string(&self) -> String {
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
