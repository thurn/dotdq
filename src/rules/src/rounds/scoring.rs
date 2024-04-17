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

use data::contract_phase_data::ContractNumber;
use data::play_phase_data::PlayPhaseData;
use data::primitive::numerics::Intel;
use data::primitive::primitives::PlayerName;

use crate::rounds::tricks;

pub fn score(data: &PlayPhaseData, player: PlayerName) -> Intel {
    let contract = data.contracts.contract_number(player);
    if tricks::won(data, player) < contract {
        Intel(0)
    } else {
        contract_value(contract)
    }
}

pub fn contract_value(number: ContractNumber) -> Intel {
    match number {
        0 => Intel(0),
        1 => Intel(10),
        2 => Intel(20),
        3 => Intel(30),
        4 => Intel(50),
        5 => Intel(100),
        6 => Intel(150),
        7 => Intel(200),
        8 => Intel(400),
        9 => Intel(700),
        10 => Intel(1000),
        11 => Intel(1500),
        12 => Intel(2000),
        _ => Intel(2500),
    }
}
