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

use crossbeam::atomic::AtomicCell;
use data::contract_phase_data::{ContractPhaseAction, ContractPhaseData};
use data::game_action::GameAction;
use data::play_phase_data::PlayPhaseData;
use data::primitives::PlayerName;
use rand::Rng;
use rules::rounds::tricks;
use tracing::info;

use crate::core::agent::AgentConfig;
use crate::game::agents;
use crate::game::agents::AgentName;
use crate::testing::run_matchup;
use crate::testing::run_matchup::Verbosity;

static AGENT_ACTION: AtomicCell<Option<GameAction>> = AtomicCell::new(None);

/// Removes & returns the next AI agent action to take, if any is available.
pub fn poll_action() -> Option<GameAction> {
    AGENT_ACTION.take()
}

pub fn initiate_selection(data: PlayPhaseData) {
    info!("Starting AI Agent search");
    rayon::spawn(move || {
        let agent = agents::get_agent(AgentName::Uct1);
        let action = agent.pick_action(
            AgentConfig {
                deadline: Instant::now() + Duration::from_secs(1),
                panic_on_search_timeout: false,
            },
            &data,
        );
        AGENT_ACTION.store(Some(action.into()));
    });
}

pub fn populate_agent_contracts(data: ContractPhaseData) {
    info!("Starting AI contract selection");
    rayon::spawn(move || {
        let mut play_phase_data = data.to_play_phase();
        run_matchup::run_match(
            AgentName::Uct1MaxTricks,
            AgentName::Uct1MaxTricks,
            &mut play_phase_data,
            100,
            Verbosity::None,
            false,
        );

        let mut west = tricks::won(&play_phase_data, PlayerName::West);
        let mut north = tricks::won(&play_phase_data, PlayerName::North);
        let mut east = tricks::won(&play_phase_data, PlayerName::East);

        while play_phase_data.contracts.contract_number(PlayerName::User) + west + north + east
            == 13
        {
            // Sum of contracts cannot be 13. Randomly vary bids until this is no longer the
            // case.
            let roll = rand::thread_rng().gen_range(1..=6);
            match roll {
                1 => west += 1,
                2 => west = west.saturating_sub(1),
                3 => north += 1,
                4 => north = north.saturating_sub(1),
                5 => east += 1,
                6 => east = east.saturating_sub(1),
                _ => panic!("Out of range"),
            }
        }

        AGENT_ACTION.store(Some(GameAction::ContractAction(
            ContractPhaseAction::SetAgentContracts { west, north, east },
        )))
    });
}
