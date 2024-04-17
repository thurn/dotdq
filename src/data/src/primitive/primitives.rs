// Copyright © Dungeon of the Diamond Queen 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use enum_iterator::Sequence;
use enumset::EnumSetType;

/// Represents the four traditional playing card suits.
///
/// Suits are ordered Clubs < Diamonds < Hearts < Spades.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Sequence, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        })
    }
}

/// Represents the standard playing card ranks, with Aces high
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Sequence, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        })
    }
}

/// Represents one of the 52 standard playing cards. Card ordering is by [Suit]
/// first and then by [Rank].
///
/// This is represented as an expanded enum so we can store hands in a bitset
/// efficiently. Changing this was a roughly 3x increase in all benchmarks.
#[derive(Hash, Ord, PartialOrd, EnumSetType)]
pub enum Card {
    TwoOfClubs,
    ThreeOfClubs,
    FourOfClubs,
    FiveOfClubs,
    SixOfClubs,
    SevenOfClubs,
    EightOfClubs,
    NineOfClubs,
    TenOfClubs,
    JackOfClubs,
    QueenOfClubs,
    KingOfClubs,
    AceOfClubs,
    TwoOfDiamonds,
    ThreeOfDiamonds,
    FourOfDiamonds,
    FiveOfDiamonds,
    SixOfDiamonds,
    SevenOfDiamonds,
    EightOfDiamonds,
    NineOfDiamonds,
    TenOfDiamonds,
    JackOfDiamonds,
    QueenOfDiamonds,
    KingOfDiamonds,
    AceOfDiamonds,
    TwoOfHearts,
    ThreeOfHearts,
    FourOfHearts,
    FiveOfHearts,
    SixOfHearts,
    SevenOfHearts,
    EightOfHearts,
    NineOfHearts,
    TenOfHearts,
    JackOfHearts,
    QueenOfHearts,
    KingOfHearts,
    AceOfHearts,
    TwoOfSpades,
    ThreeOfSpades,
    FourOfSpades,
    FiveOfSpades,
    SixOfSpades,
    SevenOfSpades,
    EightOfSpades,
    NineOfSpades,
    TenOfSpades,
    JackOfSpades,
    QueenOfSpades,
    KingOfSpades,
    AceOfSpades,
}

impl Card {
    pub fn suit(&self) -> Suit {
        match self {
            Card::TwoOfClubs => Suit::Clubs,
            Card::ThreeOfClubs => Suit::Clubs,
            Card::FourOfClubs => Suit::Clubs,
            Card::FiveOfClubs => Suit::Clubs,
            Card::SixOfClubs => Suit::Clubs,
            Card::SevenOfClubs => Suit::Clubs,
            Card::EightOfClubs => Suit::Clubs,
            Card::NineOfClubs => Suit::Clubs,
            Card::TenOfClubs => Suit::Clubs,
            Card::JackOfClubs => Suit::Clubs,
            Card::QueenOfClubs => Suit::Clubs,
            Card::KingOfClubs => Suit::Clubs,
            Card::AceOfClubs => Suit::Clubs,
            Card::TwoOfDiamonds => Suit::Diamonds,
            Card::ThreeOfDiamonds => Suit::Diamonds,
            Card::FourOfDiamonds => Suit::Diamonds,
            Card::FiveOfDiamonds => Suit::Diamonds,
            Card::SixOfDiamonds => Suit::Diamonds,
            Card::SevenOfDiamonds => Suit::Diamonds,
            Card::EightOfDiamonds => Suit::Diamonds,
            Card::NineOfDiamonds => Suit::Diamonds,
            Card::TenOfDiamonds => Suit::Diamonds,
            Card::JackOfDiamonds => Suit::Diamonds,
            Card::QueenOfDiamonds => Suit::Diamonds,
            Card::KingOfDiamonds => Suit::Diamonds,
            Card::AceOfDiamonds => Suit::Diamonds,
            Card::TwoOfHearts => Suit::Hearts,
            Card::ThreeOfHearts => Suit::Hearts,
            Card::FourOfHearts => Suit::Hearts,
            Card::FiveOfHearts => Suit::Hearts,
            Card::SixOfHearts => Suit::Hearts,
            Card::SevenOfHearts => Suit::Hearts,
            Card::EightOfHearts => Suit::Hearts,
            Card::NineOfHearts => Suit::Hearts,
            Card::TenOfHearts => Suit::Hearts,
            Card::JackOfHearts => Suit::Hearts,
            Card::QueenOfHearts => Suit::Hearts,
            Card::KingOfHearts => Suit::Hearts,
            Card::AceOfHearts => Suit::Hearts,
            Card::TwoOfSpades => Suit::Spades,
            Card::ThreeOfSpades => Suit::Spades,
            Card::FourOfSpades => Suit::Spades,
            Card::FiveOfSpades => Suit::Spades,
            Card::SixOfSpades => Suit::Spades,
            Card::SevenOfSpades => Suit::Spades,
            Card::EightOfSpades => Suit::Spades,
            Card::NineOfSpades => Suit::Spades,
            Card::TenOfSpades => Suit::Spades,
            Card::JackOfSpades => Suit::Spades,
            Card::QueenOfSpades => Suit::Spades,
            Card::KingOfSpades => Suit::Spades,
            Card::AceOfSpades => Suit::Spades,
        }
    }

    pub fn rank(&self) -> Rank {
        match self {
            Card::TwoOfClubs => Rank::Two,
            Card::ThreeOfClubs => Rank::Three,
            Card::FourOfClubs => Rank::Four,
            Card::FiveOfClubs => Rank::Five,
            Card::SixOfClubs => Rank::Six,
            Card::SevenOfClubs => Rank::Seven,
            Card::EightOfClubs => Rank::Eight,
            Card::NineOfClubs => Rank::Nine,
            Card::TenOfClubs => Rank::Ten,
            Card::JackOfClubs => Rank::Jack,
            Card::QueenOfClubs => Rank::Queen,
            Card::KingOfClubs => Rank::King,
            Card::AceOfClubs => Rank::Ace,
            Card::TwoOfDiamonds => Rank::Two,
            Card::ThreeOfDiamonds => Rank::Three,
            Card::FourOfDiamonds => Rank::Four,
            Card::FiveOfDiamonds => Rank::Five,
            Card::SixOfDiamonds => Rank::Six,
            Card::SevenOfDiamonds => Rank::Seven,
            Card::EightOfDiamonds => Rank::Eight,
            Card::NineOfDiamonds => Rank::Nine,
            Card::TenOfDiamonds => Rank::Ten,
            Card::JackOfDiamonds => Rank::Jack,
            Card::QueenOfDiamonds => Rank::Queen,
            Card::KingOfDiamonds => Rank::King,
            Card::AceOfDiamonds => Rank::Ace,
            Card::TwoOfHearts => Rank::Two,
            Card::ThreeOfHearts => Rank::Three,
            Card::FourOfHearts => Rank::Four,
            Card::FiveOfHearts => Rank::Five,
            Card::SixOfHearts => Rank::Six,
            Card::SevenOfHearts => Rank::Seven,
            Card::EightOfHearts => Rank::Eight,
            Card::NineOfHearts => Rank::Nine,
            Card::TenOfHearts => Rank::Ten,
            Card::JackOfHearts => Rank::Jack,
            Card::QueenOfHearts => Rank::Queen,
            Card::KingOfHearts => Rank::King,
            Card::AceOfHearts => Rank::Ace,
            Card::TwoOfSpades => Rank::Two,
            Card::ThreeOfSpades => Rank::Three,
            Card::FourOfSpades => Rank::Four,
            Card::FiveOfSpades => Rank::Five,
            Card::SixOfSpades => Rank::Six,
            Card::SevenOfSpades => Rank::Seven,
            Card::EightOfSpades => Rank::Eight,
            Card::NineOfSpades => Rank::Nine,
            Card::TenOfSpades => Rank::Ten,
            Card::JackOfSpades => Rank::Jack,
            Card::QueenOfSpades => Rank::Queen,
            Card::KingOfSpades => Rank::King,
            Card::AceOfSpades => Rank::Ace,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        match suit {
            Suit::Clubs => match rank {
                Rank::Two => Card::TwoOfClubs,
                Rank::Three => Card::ThreeOfClubs,
                Rank::Four => Card::FourOfClubs,
                Rank::Five => Card::FiveOfClubs,
                Rank::Six => Card::SixOfClubs,
                Rank::Seven => Card::SevenOfClubs,
                Rank::Eight => Card::EightOfClubs,
                Rank::Nine => Card::NineOfClubs,
                Rank::Ten => Card::TenOfClubs,
                Rank::Jack => Card::JackOfClubs,
                Rank::Queen => Card::QueenOfClubs,
                Rank::King => Card::KingOfClubs,
                Rank::Ace => Card::AceOfClubs,
            },
            Suit::Diamonds => match rank {
                Rank::Two => Card::TwoOfDiamonds,
                Rank::Three => Card::ThreeOfDiamonds,
                Rank::Four => Card::FourOfDiamonds,
                Rank::Five => Card::FiveOfDiamonds,
                Rank::Six => Card::SixOfDiamonds,
                Rank::Seven => Card::SevenOfDiamonds,
                Rank::Eight => Card::EightOfDiamonds,
                Rank::Nine => Card::NineOfDiamonds,
                Rank::Ten => Card::TenOfDiamonds,
                Rank::Jack => Card::JackOfDiamonds,
                Rank::Queen => Card::QueenOfDiamonds,
                Rank::King => Card::KingOfDiamonds,
                Rank::Ace => Card::AceOfDiamonds,
            },
            Suit::Hearts => match rank {
                Rank::Two => Card::TwoOfHearts,
                Rank::Three => Card::ThreeOfHearts,
                Rank::Four => Card::FourOfHearts,
                Rank::Five => Card::FiveOfHearts,
                Rank::Six => Card::SixOfHearts,
                Rank::Seven => Card::SevenOfHearts,
                Rank::Eight => Card::EightOfHearts,
                Rank::Nine => Card::NineOfHearts,
                Rank::Ten => Card::TenOfHearts,
                Rank::Jack => Card::JackOfHearts,
                Rank::Queen => Card::QueenOfHearts,
                Rank::King => Card::KingOfHearts,
                Rank::Ace => Card::AceOfHearts,
            },
            Suit::Spades => match rank {
                Rank::Two => Card::TwoOfSpades,
                Rank::Three => Card::ThreeOfSpades,
                Rank::Four => Card::FourOfSpades,
                Rank::Five => Card::FiveOfSpades,
                Rank::Six => Card::SixOfSpades,
                Rank::Seven => Card::SevenOfSpades,
                Rank::Eight => Card::EightOfSpades,
                Rank::Nine => Card::NineOfSpades,
                Rank::Ten => Card::TenOfSpades,
                Rank::Jack => Card::JackOfSpades,
                Rank::Queen => Card::QueenOfSpades,
                Rank::King => Card::KingOfSpades,
                Rank::Ace => Card::AceOfSpades,
            },
        }
    }
}

/// Represents one of the four hands in a game.
#[derive(Hash, Ord, PartialOrd, EnumSetType, Sequence)]
pub enum PlayerName {
    User,
    West,
    North,
    East,
}

impl PlayerName {
    pub fn is_user(&self) -> bool {
        *self == Self::User
    }
}

impl Debug for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PlayerName::User => "U",
            PlayerName::West => "W",
            PlayerName::North => "N",
            PlayerName::East => "E",
        })
    }
}

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PlayerName::User => "USER",
            PlayerName::West => "WEST",
            PlayerName::North => "NORTH",
            PlayerName::East => "EAST",
        })
    }
}

impl PlayerName {
    /// Returns the next position in turn sequence after this one
    pub fn next(&self) -> Self {
        match self {
            Self::User => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
            Self::East => Self::User,
        }
    }

    /// Returns true if this player name corresponds to an AI player
    pub fn is_agent(&self) -> bool {
        *self != Self::User
    }
}
