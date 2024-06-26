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

use data::contract_phase_data::{ContractPhaseAction, ContractPhaseData, ContractPhaseStep};
use data::primitive::primitives::PlayerName;
use data::round_data::RoundData;

use crate::contract_phase::contract_phase_queries;
use crate::rounds::start_play_phase;

pub fn handle_action(
    data: &mut ContractPhaseData,
    player: PlayerName,
    action: ContractPhaseAction,
) -> Option<RoundData> {
    assert!(
        contract_phase_queries::can_perform_action(data, player, action),
        "Cannot perform action {action:?}"
    );
    match action {
        ContractPhaseAction::SetUserContract(number) => {
            *data.contracts.contract_number_mut(player) = number;
        }
        ContractPhaseAction::AcceptUserContract => {
            data.step = ContractPhaseStep::AwaitingAgentContracts;
        }
        ContractPhaseAction::SetAgentContracts { west, north, east } => {
            *data.contracts.contract_number_mut(PlayerName::West) = west;
            *data.contracts.contract_number_mut(PlayerName::North) = north;
            *data.contracts.contract_number_mut(PlayerName::East) = east;
            data.step = ContractPhaseStep::ReadyToStart;
        }
        ContractPhaseAction::StartPlayPhase => {
            return Some(RoundData::PlayPhase(start_play_phase::run(data.clone())))
        }
    }

    None
}
