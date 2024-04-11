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
use std::fmt::{Debug, Formatter};

use enum_iterator::Sequence;
use enumset::EnumSetType;

use crate::primitives::Suit;

#[derive(Hash, Ord, PartialOrd, EnumSetType, Sequence)]
pub enum Bid {
    SixClubs,
    SixDiamonds,
    SixHearts,
    SixSpades,
    SixNoTrump,
    SevenClubs,
    SevenDiamonds,
    SevenHearts,
    SevenSpades,
    SevenNoTrump,
    EightClubs,
    EightDiamonds,
    EightHearts,
    EightSpades,
    EightNoTrump,
    NineClubs,
    NineDiamonds,
    NineHearts,
    NineSpades,
    NineNoTrump,
    TenClubs,
    TenDiamonds,
    TenHearts,
    TenSpades,
    TenNoTrump,
    ElevenClubs,
    ElevenDiamonds,
    ElevenHearts,
    ElevenSpades,
    ElevenNoTrump,
    TwelveClubs,
    TwelveDiamonds,
    TwelveHearts,
    TwelveSpades,
    TwelveNoTrump,
    ThirteenClubs,
    ThirteenDiamonds,
    ThirteenHearts,
    ThirteenSpades,
    ThirteenNoTrump,
}

impl Debug for Bid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "B{:?}{:?}", self.bid_number().as_u32(), self.suit())
    }
}

impl Bid {
    pub fn new(number: BidNumber, suit: Option<Suit>) -> Self {
        match suit {
            None => match number {
                BidNumber::Six => Bid::SixNoTrump,
                BidNumber::Seven => Bid::SevenNoTrump,
                BidNumber::Eight => Bid::EightNoTrump,
                BidNumber::Nine => Bid::NineNoTrump,
                BidNumber::Ten => Bid::TenNoTrump,
                BidNumber::Eleven => Bid::ElevenNoTrump,
                BidNumber::Twelve => Bid::TwelveNoTrump,
                BidNumber::Thirteen => Bid::ThirteenNoTrump,
            },
            Some(Suit::Clubs) => match number {
                BidNumber::Six => Bid::SixClubs,
                BidNumber::Seven => Bid::SevenClubs,
                BidNumber::Eight => Bid::EightClubs,
                BidNumber::Nine => Bid::NineClubs,
                BidNumber::Ten => Bid::TenClubs,
                BidNumber::Eleven => Bid::ElevenClubs,
                BidNumber::Twelve => Bid::TwelveClubs,
                BidNumber::Thirteen => Bid::ThirteenClubs,
            },
            Some(Suit::Diamonds) => match number {
                BidNumber::Six => Bid::SixDiamonds,
                BidNumber::Seven => Bid::SevenDiamonds,
                BidNumber::Eight => Bid::EightDiamonds,
                BidNumber::Nine => Bid::NineDiamonds,
                BidNumber::Ten => Bid::TenDiamonds,
                BidNumber::Eleven => Bid::ElevenDiamonds,
                BidNumber::Twelve => Bid::TwelveDiamonds,
                BidNumber::Thirteen => Bid::ThirteenDiamonds,
            },
            Some(Suit::Hearts) => match number {
                BidNumber::Six => Bid::SixHearts,
                BidNumber::Seven => Bid::SevenHearts,
                BidNumber::Eight => Bid::EightHearts,
                BidNumber::Nine => Bid::NineHearts,
                BidNumber::Ten => Bid::TenHearts,
                BidNumber::Eleven => Bid::ElevenHearts,
                BidNumber::Twelve => Bid::TwelveHearts,
                BidNumber::Thirteen => Bid::ThirteenHearts,
            },
            Some(Suit::Spades) => match number {
                BidNumber::Six => Bid::SixSpades,
                BidNumber::Seven => Bid::SevenSpades,
                BidNumber::Eight => Bid::EightSpades,
                BidNumber::Nine => Bid::NineSpades,
                BidNumber::Ten => Bid::TenSpades,
                BidNumber::Eleven => Bid::ElevenSpades,
                BidNumber::Twelve => Bid::TwelveSpades,
                BidNumber::Thirteen => Bid::ThirteenSpades,
            },
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            Bid::SixClubs => Some(Suit::Clubs),
            Bid::SixDiamonds => Some(Suit::Diamonds),
            Bid::SixHearts => Some(Suit::Hearts),
            Bid::SixSpades => Some(Suit::Spades),
            Bid::SixNoTrump => None,
            Bid::SevenClubs => Some(Suit::Clubs),
            Bid::SevenDiamonds => Some(Suit::Diamonds),
            Bid::SevenHearts => Some(Suit::Hearts),
            Bid::SevenSpades => Some(Suit::Spades),
            Bid::SevenNoTrump => None,
            Bid::EightClubs => Some(Suit::Clubs),
            Bid::EightDiamonds => Some(Suit::Diamonds),
            Bid::EightHearts => Some(Suit::Hearts),
            Bid::EightSpades => Some(Suit::Spades),
            Bid::EightNoTrump => None,
            Bid::NineClubs => Some(Suit::Clubs),
            Bid::NineDiamonds => Some(Suit::Diamonds),
            Bid::NineHearts => Some(Suit::Hearts),
            Bid::NineSpades => Some(Suit::Spades),
            Bid::NineNoTrump => None,
            Bid::TenClubs => Some(Suit::Clubs),
            Bid::TenDiamonds => Some(Suit::Diamonds),
            Bid::TenHearts => Some(Suit::Hearts),
            Bid::TenSpades => Some(Suit::Spades),
            Bid::TenNoTrump => None,
            Bid::ElevenClubs => Some(Suit::Clubs),
            Bid::ElevenDiamonds => Some(Suit::Diamonds),
            Bid::ElevenHearts => Some(Suit::Hearts),
            Bid::ElevenSpades => Some(Suit::Spades),
            Bid::ElevenNoTrump => None,
            Bid::TwelveClubs => Some(Suit::Clubs),
            Bid::TwelveDiamonds => Some(Suit::Diamonds),
            Bid::TwelveHearts => Some(Suit::Hearts),
            Bid::TwelveSpades => Some(Suit::Spades),
            Bid::TwelveNoTrump => None,
            Bid::ThirteenClubs => Some(Suit::Clubs),
            Bid::ThirteenDiamonds => Some(Suit::Diamonds),
            Bid::ThirteenHearts => Some(Suit::Hearts),
            Bid::ThirteenSpades => Some(Suit::Spades),
            Bid::ThirteenNoTrump => None,
        }
    }

    pub fn bid_number(&self) -> BidNumber {
        match self {
            Bid::SixClubs => BidNumber::Six,
            Bid::SixDiamonds => BidNumber::Six,
            Bid::SixHearts => BidNumber::Six,
            Bid::SixSpades => BidNumber::Six,
            Bid::SixNoTrump => BidNumber::Six,
            Bid::SevenClubs => BidNumber::Seven,
            Bid::SevenDiamonds => BidNumber::Seven,
            Bid::SevenHearts => BidNumber::Seven,
            Bid::SevenSpades => BidNumber::Seven,
            Bid::SevenNoTrump => BidNumber::Seven,
            Bid::EightClubs => BidNumber::Eight,
            Bid::EightDiamonds => BidNumber::Eight,
            Bid::EightHearts => BidNumber::Eight,
            Bid::EightSpades => BidNumber::Eight,
            Bid::EightNoTrump => BidNumber::Eight,
            Bid::NineClubs => BidNumber::Nine,
            Bid::NineDiamonds => BidNumber::Nine,
            Bid::NineHearts => BidNumber::Nine,
            Bid::NineSpades => BidNumber::Nine,
            Bid::NineNoTrump => BidNumber::Nine,
            Bid::TenClubs => BidNumber::Ten,
            Bid::TenDiamonds => BidNumber::Ten,
            Bid::TenHearts => BidNumber::Ten,
            Bid::TenSpades => BidNumber::Ten,
            Bid::TenNoTrump => BidNumber::Ten,
            Bid::ElevenClubs => BidNumber::Eleven,
            Bid::ElevenDiamonds => BidNumber::Eleven,
            Bid::ElevenHearts => BidNumber::Eleven,
            Bid::ElevenSpades => BidNumber::Eleven,
            Bid::ElevenNoTrump => BidNumber::Eleven,
            Bid::TwelveClubs => BidNumber::Twelve,
            Bid::TwelveDiamonds => BidNumber::Twelve,
            Bid::TwelveHearts => BidNumber::Twelve,
            Bid::TwelveSpades => BidNumber::Twelve,
            Bid::TwelveNoTrump => BidNumber::Twelve,
            Bid::ThirteenClubs => BidNumber::Thirteen,
            Bid::ThirteenDiamonds => BidNumber::Thirteen,
            Bid::ThirteenHearts => BidNumber::Thirteen,
            Bid::ThirteenSpades => BidNumber::Thirteen,
            Bid::ThirteenNoTrump => BidNumber::Thirteen,
        }
    }
}

#[derive(Hash, Ord, PartialOrd, EnumSetType, Debug)]
pub enum BidNumber {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
}

impl BidNumber {
    pub fn as_u32(&self) -> u32 {
        match self {
            BidNumber::Six => 6,
            BidNumber::Seven => 7,
            BidNumber::Eight => 8,
            BidNumber::Nine => 9,
            BidNumber::Ten => 10,
            BidNumber::Eleven => 11,
            BidNumber::Twelve => 12,
            BidNumber::Thirteen => 13,
        }
    }
}
