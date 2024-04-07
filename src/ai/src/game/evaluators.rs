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

use data::play_phase_data::PlayPhaseData;
use data::primitives::PlayerName;
use rules::play_phase_queries;

use crate::core::state_evaluator::StateEvaluator;

pub struct TrickEvaluator;

impl StateEvaluator<PlayPhaseData> for TrickEvaluator {
    fn evaluate(&self, data: &PlayPhaseData, player: PlayerName) -> i32 {
        play_phase_queries::tricks_won(data, player)
            - play_phase_queries::tricks_won(data, player.opponent())
    }
}
