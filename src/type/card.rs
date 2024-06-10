
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
    known: bool,
}

impl Card {
    fn to_string(&self) -> String {
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
