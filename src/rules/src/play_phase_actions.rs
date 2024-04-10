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

use data::play_phase_data::{CompletedTrick, PlayPhaseAction, PlayPhaseData, PlayedCard};
use data::primitives::{Card, HandIdentifier, PlayerName};

use crate::play_phase_queries;

pub fn handle_action(data: &mut PlayPhaseData, action: PlayPhaseAction) {
    assert!(
        play_phase_queries::can_perform_action(data, action),
        "Cannot perform action {action:?}"
    );
    match action {
        PlayPhaseAction::PlayCard(player, hand, card) => play_card(data, player, hand, card),
    }
}

/// Plays the indicated [Card] from the hand identified by [HandIdentifier] if
/// it is currently legal to do so.
fn play_card(data: &mut PlayPhaseData, _: PlayerName, hand: HandIdentifier, card: Card) {
    data.hands.get_mut(&hand).unwrap().remove(&card);
    if data.current_trick.cards.len() >= 4 {
        data.current_trick.cards.clear();
    }
    data.current_trick.cards.push(PlayedCard { played_by: hand, card });
    if data.current_trick.cards.len() == 4 {
        let trick = data.current_trick.clone();
        let winner = play_phase_queries::trick_winner(&trick);
        data.completed_tricks.push(CompletedTrick { trick, winner });
    }
}
