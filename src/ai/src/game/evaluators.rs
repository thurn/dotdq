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
use data::primitive::primitives::PlayerName;
use rules::rounds::tricks;

use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::state_evaluator::StateEvaluator;

pub struct TrickEvaluator;

impl StateEvaluator<PlayPhaseData> for TrickEvaluator {
    fn evaluate(&self, data: &PlayPhaseData, player: PlayerName) -> i32 {
        match data.status() {
            GameStatus::InProgress { .. } => tricks::won(data, player) as i32,
            GameStatus::Completed { scores } => scores[&player],
        }
    }
}

pub struct MaxTricksEvaluator;

impl StateEvaluator<PlayPhaseData> for MaxTricksEvaluator {
    fn evaluate(&self, data: &PlayPhaseData, player: PlayerName) -> i32 {
        tricks::won(data, player) as i32
    }
}
