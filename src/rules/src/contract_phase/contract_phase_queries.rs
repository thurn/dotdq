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
use data::primitives::PlayerName;

/// Returns true if the indicated [ContractPhaseAction] is currently legal to
/// take
pub fn can_perform_action(
    data: &ContractPhaseData,
    player: PlayerName,
    action: ContractPhaseAction,
) -> bool {
    if player != PlayerName::User {
        return false;
    }
    match action {
        ContractPhaseAction::AcceptUserContract => {
            data.step == ContractPhaseStep::AwaitingUserContact
        }
        ContractPhaseAction::SetUserContract(_) => {
            data.step == ContractPhaseStep::AwaitingUserContact
        }
        ContractPhaseAction::SetAgentContracts { west, north, east } => {
            data.step == ContractPhaseStep::AwaitingAgentContracts
                // Contract numbers cannot sum to 13
                && west + north + east + data.contracts.contract_number(PlayerName::User) != 13
        }
        ContractPhaseAction::StartPlayPhase => data.step == ContractPhaseStep::ReadyToStart,
    }
}
