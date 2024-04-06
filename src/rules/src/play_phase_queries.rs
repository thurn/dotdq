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

use data::play_phase_data::{PlayPhaseAction, PlayPhaseData, Trick};
use data::primitives::HandIdentifier;

/// Returns the [HandIdentifier] to next play a card during a round.
pub fn next_to_play(data: &PlayPhaseData) -> HandIdentifier {
    match data.current_trick.cards.len() {
        0 => {
            if let Some(last) = data.completed_tricks.last() {
                trick_winner(&last.trick)
            } else {
                data.contract.declarer.primary_hand()
            }
        }
        1..=3 => data.current_trick.cards.last().unwrap().played_by.next(),
        4 => trick_winner(&data.current_trick),
        _ => panic!("Invalid trick size"),
    }
}

/// Returns the [HandIdentifier] which won a given trick
pub fn trick_winner(trick: &Trick) -> HandIdentifier {
    let mut cards = trick.cards.clone();
    cards.sort_by_key(|played| played.card);
    cards.last().unwrap().played_by
}

pub fn can_perform_action(data: &PlayPhaseData, action: PlayPhaseAction) -> bool {
    match action {
        PlayPhaseAction::PlayCard(player, hand, card) => {
            if next_to_play(data) != hand {
                return false;
            }
            player.owns_hand(hand) && data.hands.get(&hand).unwrap().contains(&card)
        }
    }
}
