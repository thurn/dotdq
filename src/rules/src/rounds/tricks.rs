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

use std::cmp::Ordering;

use data::play_phase_data::{PlayPhaseData, Trick, TrickNumber};
use data::primitives::{Card, PlayerName, Suit};

/// Returns true if the [PlayerName] player is currently able to lead a card.
pub fn has_lead(data: &PlayPhaseData, player_name: PlayerName) -> bool {
    if !data.current_trick.cards.is_empty() {
        return false;
    }

    data.completed_tricks.last().map_or(PlayerName::User, |t| t.winner) == player_name
}

/// Returns the [TrickNumber] for the current trick.
///
/// When waiting for a card to be lead after a trick is completed, this is the
/// number for the next trick.
pub fn current_number(data: &PlayPhaseData) -> TrickNumber {
    data.completed_tricks.len()
}

/// Returns the number of tricks the [PlayerName] player has won in the
/// provided game so far.
pub fn won(data: &PlayPhaseData, player: PlayerName) -> usize {
    data.completed_tricks.iter().filter(|t| t.winner == player).count()
}

/// Returns the [PlayerName] which won a given trick.
///
/// Panics if the provided trick is empty.
pub fn winner(data: &PlayPhaseData, trick: &Trick) -> PlayerName {
    let suit = suit(trick).expect("Trick was empty");
    let winner = trick
        .cards
        .iter()
        .max_by(|a, b| card_ordering(data, suit, a.card, b.card))
        .expect("Trick was empty")
        .played_by;
    data.programs.current_delegates.trick_winner.run_query(data, &current_number(data), winner)
}

pub fn card_ordering(data: &PlayPhaseData, trick_suit: Suit, left: Card, right: Card) -> Ordering {
    match data.trump {
        Some(trump) if left.suit() != right.suit() && left.suit() == trump => Ordering::Greater,
        Some(trump) if left.suit() != right.suit() && right.suit() == trump => Ordering::Less,
        _ if left.suit() != right.suit() && left.suit() == trick_suit => Ordering::Greater,
        _ if left.suit() != right.suit() && right.suit() == trick_suit => Ordering::Less,
        _ => left.cmp(&right),
    }
}

/// Returns the [Suit] being used for the provided trick, or None if the trick
/// is empty.
pub fn suit(trick: &Trick) -> Option<Suit> {
    Some(trick.cards.first()?.card.suit())
}
