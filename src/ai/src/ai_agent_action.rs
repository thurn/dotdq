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

use std::time::{Duration, Instant};

use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};

use crate::core::agent::AgentConfig;
use crate::game::agents;
use crate::game::agents::AgentName;

/// Selects the next action to take for the currently-configured opponent AI
/// agent.
pub fn select(data: &PlayPhaseData) -> PlayPhaseAction {
    let agent = agents::get_agent(AgentName::Uct1);
    agent.pick_action(
        AgentConfig {
            deadline: Instant::now() + Duration::from_secs(3),
            panic_on_search_timeout: true,
        },
        data,
    )

    // play_phase_queries::legal_actions(data, PlayerName::Opponent)
    //     .next()
    //     .expect("No legal actions available")
}
