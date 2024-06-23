// AdrtH (C) 2024
use raylib::ffi::LoadTexture;
use raylib::ffi::Rectangle;
use raylib::ffi::Texture2D;
use std::collections::HashMap;
use std::hash::Hash;
use std::os::raw::c_char;

pub static mut CARD_BACK: Option<Texture2D> = None;
pub static mut CARD_FRONTS: Option<HashMap<Card, Texture2D>> = None;

pub unsafe fn load_cards_texture() {
    CARD_BACK = Some(LoadTexture(
        "cards/svg_playing_cards/backs/png_96_dpi/red2.png\0".as_ptr() as *const c_char,
    ));
    CARD_FRONTS = Some(HashMap::new());
    for value in 1..=13 {
        for suit in [Suit::DIAMOND, Suit::CLUB, Suit::SPADE, Suit::HEART] {
            let card = Card {
                suit,
                value,
                known: true,
            };
            let texture = LoadTexture(match_card_path_texture(&card).as_ptr() as *const c_char);
            if let Some(ref mut map) = CARD_FRONTS {
                map.insert(card, texture);
            }
        }
    }
}

pub fn match_card_path_texture(card: &Card) -> String {
    let mut path = "cards/svg_playing_cards/fronts/png_96_dpi/".to_string();
    let color = match card.suit {
        Suit::HEART => "hearts_",
        Suit::SPADE => "spades_",
        Suit::CLUB => "clubs_",
        Suit::DIAMOND => "diamonds_",
    };
    path += color;
    let string_val = card.value.to_string();
    let value = match card.value {
        11 => "jack",
        12 => "queen",
        13 => "king",
        1 => "ace",
        _ => &string_val,
    };
    path += value;
    path += ".png\0";
    path
}

pub fn match_card_texture(card: &Card) -> Texture2D {
    let texture = unsafe {
        let map = CARD_FRONTS.clone().unwrap();
        map.get(card).unwrap().clone()
    };
    texture
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub enum Suit {
    CLUB,
    DIAMOND,
    SPADE,
    HEART,
}

impl Suit {
    // red is true, because I had to make a choice and red is my favorite color
    pub fn to_color_as_bool(self) -> bool {
        return self == Suit::DIAMOND || self == Suit::HEART;
    }
}

#[derive(Clone, Copy, Hash)]
pub struct Card {
    pub value: i16,
    pub suit: Suit,
    pub known: bool,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

impl Eq for Card {}

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
        format!("{},{},{}", s, val, self.known)
    }

    pub fn is_stackable(card_high: Self, card_low: Self) -> bool {
        card_high.value == card_low.value + 1
            && card_high.suit.to_color_as_bool() != card_low.suit.to_color_as_bool()
    }
}

pub struct VisualCard<'a> {
    pub card: &'a Card,
    pub pos: Rectangle,
    pub collision: Rectangle,
}
