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

use std::collections::{HashMap, HashSet};

use data::contract_phase_data::ContractPhaseData;
use data::delegate_data::{PlayPhaseDelegates, ProgramId};
use data::play_phase_data::{PlayPhaseData, Trick};
use data::primitive::primitives::PlayerName;
use data::program_data::ProgramData;

use crate::program::programs;

pub fn run(data: ContractPhaseData) -> PlayPhaseData {
    let mut delegates = PlayPhaseDelegates::default();
    for player in enum_iterator::all::<PlayerName>() {
        for program in data.programs.all_programs.get(&player).unwrap_or(&vec![]) {
            let definition = programs::get(*program);
            if let Some(play_phase) = definition.play_phase {
                let id = ProgramId::new(*program, player);
                play_phase(&mut delegates);
                delegates.set_current_id(id);
            }
        }
    }

    PlayPhaseData {
        current_trick: Trick::default(),
        turn: Some(PlayerName::User),
        completed_tricks: vec![],
        trump: data.trump,
        contracts: data.contracts,
        hands: data.hands,
        programs: ProgramData {
            current_delegates: delegates,
            program_state: HashMap::new(),
            all_programs: data.programs.all_programs,
            activated: HashSet::new(),
        },
    }
}
