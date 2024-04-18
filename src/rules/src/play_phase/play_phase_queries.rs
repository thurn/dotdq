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

use data::delegate_data::{HasPrograms, ProgramId};
use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitive::primitives::{PlayerName, Suit};

use crate::rounds::cards;

/// Returns true if the indicated [PlayPhaseAction] is currently legal to take
pub fn can_perform_action(
    data: &PlayPhaseData,
    player: PlayerName,
    action: PlayPhaseAction,
) -> bool {
    match action {
        PlayPhaseAction::PlayCard(card) => cards::can_play(data, player, card),
        PlayPhaseAction::ActivateProgram(program) => data.activation_state(program).can_activate(),
    }
}

/// Returns an iterator over actions the indicated `player` can take in the
/// current game state.
pub fn legal_actions(
    data: &PlayPhaseData,
    player: PlayerName,
) -> impl Iterator<Item = PlayPhaseAction> + '_ {
    data.hands
        .hand(player)
        .iter()
        .map(PlayPhaseAction::PlayCard)
        .chain(
            data.programs
                .for_player(player)
                .map(move |p| PlayPhaseAction::ActivateProgram(ProgramId::new(p, player))),
        )
        .filter(move |&action| can_perform_action(data, player, action))
}

/// Returns the number of cards of the given [Suit] in the indicated hand.
pub fn suit_count(data: &PlayPhaseData, hand: PlayerName, suit: Suit) -> usize {
    data.hands.hand(hand).iter().filter(|card| card.suit() == suit).count()
}
