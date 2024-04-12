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

use std::fmt::{Debug, Formatter};

use enumset::EnumSet;

use crate::contract_phase_data::Contracts;
use crate::primitives::{Card, PlayerName, Suit};

#[derive(Debug, Clone)]
pub struct PlayPhaseData {
    pub current_trick: Trick,
    pub completed_tricks: Vec<CompletedTrick>,
    pub trump: Option<Suit>,
    pub contracts: Contracts,
    pub hands: Hands,
}

#[derive(Debug, Clone)]
pub struct Hands {
    north: EnumSet<Card>,
    east: EnumSet<Card>,
    south: EnumSet<Card>,
    west: EnumSet<Card>,
}

impl Hands {
    pub fn new(
        north: EnumSet<Card>,
        east: EnumSet<Card>,
        south: EnumSet<Card>,
        west: EnumSet<Card>,
    ) -> Self {
        Self { north, east, south, west }
    }

    pub fn hand(&self, identifier: PlayerName) -> EnumSet<Card> {
        match identifier {
            PlayerName::North => self.north,
            PlayerName::East => self.east,
            PlayerName::User => self.south,
            PlayerName::West => self.west,
        }
    }

    pub fn hand_mut(&mut self, identifier: PlayerName) -> &mut EnumSet<Card> {
        match identifier {
            PlayerName::North => &mut self.north,
            PlayerName::East => &mut self.east,
            PlayerName::User => &mut self.south,
            PlayerName::West => &mut self.west,
        }
    }

    pub fn all_empty(&self) -> bool {
        self.north.is_empty()
            && self.east.is_empty()
            && self.south.is_empty()
            && self.west.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct CompletedTrick {
    /// Cards which were played in this trick.
    pub trick: Trick,
    /// Player who won this trick
    pub winner: PlayerName,
}

#[derive(Debug, Clone, Default)]
pub struct Trick {
    /// Cards played in this trick, in sequence
    pub cards: Vec<PlayedCard>,
}

/// Represents a card played to a trick
#[derive(Debug, Clone, Copy)]
pub struct PlayedCard {
    /// Player who played this card
    pub played_by: PlayerName,
    /// Card which was played
    pub card: Card,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayPhaseAction {
    PlayCard(Card),
}

impl Debug for PlayPhaseAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayPhaseAction::PlayCard(card) => {
                write!(f, "Play {:?}", card)
            }
        }
    }
}
