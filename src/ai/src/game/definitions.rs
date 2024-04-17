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

use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitive::primitives::PlayerName;
use rules::play_phase::{play_phase_actions, play_phase_queries};
use rules::rounds::scoring;

use crate::core::game_state_node::{GameStateNode, GameStatus};

impl GameStateNode for PlayPhaseData {
    type Action = PlayPhaseAction;
    type PlayerName = PlayerName;

    fn make_copy(&self) -> Self {
        self.clone()
    }

    fn status(&self) -> GameStatus<Self::PlayerName> {
        if let Some(p) = play_phase_queries::current_turn(self) {
            GameStatus::InProgress { current_turn: p }
        } else {
            GameStatus::Completed {
                scores: enum_iterator::all::<PlayerName>()
                    .map(|player| (player, scoring::score(self, player).as_i32()))
                    .collect(),
            }
        }
    }

    fn legal_actions<'a>(
        &'a self,
        player: Self::PlayerName,
    ) -> Box<dyn Iterator<Item = Self::Action> + 'a> {
        Box::new(play_phase_queries::legal_actions(self, player))
    }

    fn execute_action(&mut self, player_name: Self::PlayerName, action: Self::Action) {
        play_phase_actions::handle_action(self, player_name, action);
    }
}
