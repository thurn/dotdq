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
use crate::delegate_data::{Context, HasPrograms, PlayPhaseDelegates, ProgramId, ProgramState};
use crate::game_action::GameAction;
use crate::primitive::primitives::{Card, PlayerName, Suit};
use crate::program_data::ProgramData;

pub type TrickNumber = usize;

#[derive(Clone)]
pub struct PlayPhaseData {
    pub current_trick: Trick,
    /// Player who is next to play, or None if this game has ended.
    pub turn: Option<PlayerName>,
    pub completed_tricks: Vec<CompletedTrick>,
    pub trump: Option<Suit>,
    pub contracts: Contracts,
    pub hands: Hands,
    pub programs: ProgramData<PlayPhaseDelegates>,
}

impl PlayPhaseData {
    /// Returns true if it is this player's turn in the round
    pub fn is_turn(&self, turn: PlayerName) -> bool {
        self.turn == Some(turn)
    }
}

impl HasPrograms for PlayPhaseData {
    fn get_state(&self, id: &ProgramId) -> Option<ProgramState> {
        self.programs.program_state.get(id).copied()
    }

    fn set_state(&mut self, id: ProgramId, state: Option<ProgramState>) {
        if let Some(s) = state {
            self.programs.program_state.insert(id, s);
        } else {
            self.programs.program_state.remove(&id);
        }
    }

    fn can_activate(&self, program: ProgramId) -> bool {
        self.programs.current_delegates.can_activate.run_query(self, program, false)
    }

    fn activate(&mut self, program: ProgramId) {
        let function = self.programs.current_delegates.activated.get_mutation_fn(program);
        let mut context = Context { id: program, state: self.get_state(&program) };
        function(self, &mut context);
        self.set_state(program, context.state);
    }
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

impl Trick {
    /// Returns true if cards have started being played to this trick
    pub fn is_started(&self) -> bool {
        !self.cards.is_empty()
    }
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
    ActivateProgram(ProgramId),
}

impl From<PlayPhaseAction> for GameAction {
    fn from(value: PlayPhaseAction) -> Self {
        GameAction::PlayAction(value)
    }
}

impl Debug for PlayPhaseAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayPhaseAction::PlayCard(card) => {
                write!(f, "Play {:?}", card)
            }
            PlayPhaseAction::ActivateProgram(program) => {
                write!(f, "Activate {:?}", program.name)
            }
        }
    }
}
