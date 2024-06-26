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

use data::delegate_data::HasPrograms;
use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitive::primitives::PlayerName;

use crate::play_phase::play_phase_queries;
use crate::rounds::cards;

pub fn handle_action(data: &mut PlayPhaseData, player: PlayerName, action: PlayPhaseAction) {
    assert!(
        play_phase_queries::can_perform_action(data, player, action),
        "Cannot perform action {action:?}"
    );
    match action {
        PlayPhaseAction::PlayCard(card) => cards::play_card(data, player, card),
        PlayPhaseAction::ActivateProgram(program) => data.activate(program),
    }
}
