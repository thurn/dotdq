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

use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::state_evaluator::StateEvaluator;

/// Evaluator which returns -1 for a loss, 1 for a win, and 0 otherwise
pub struct WinLossEvaluator;

impl<TNode: GameStateNode> StateEvaluator<TNode> for WinLossEvaluator {
    fn evaluate(&self, state: &TNode, player: TNode::PlayerName) -> i32 {
        match state.status() {
            GameStatus::InProgress { .. } => 0,
            GameStatus::Completed { winner } if winner == player => 1,
            _ => -1,
        }
    }
}
