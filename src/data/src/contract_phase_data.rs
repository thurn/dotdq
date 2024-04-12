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

use std::fmt::Debug;

use crate::primitives::{PlayerName, Suit};

pub type ContractNumber = usize;

#[derive(Debug, Clone)]
pub struct ContractPhaseData {
    /// Trump suit to use for this round
    pub trump: Option<Suit>,

    user_contract: ContractNumber,
    west_contract: ContractNumber,
    north_contract: ContractNumber,
    east_contract: ContractNumber,
}

impl ContractPhaseData {
    pub fn new(trump: Option<Suit>) -> Self {
        Self { trump, user_contract: 0, north_contract: 0, east_contract: 0, west_contract: 0 }
    }

    pub fn contract_number(&self, player: PlayerName) -> ContractNumber {
        match player {
            PlayerName::User => self.user_contract,
            PlayerName::West => self.west_contract,
            PlayerName::North => self.north_contract,
            PlayerName::East => self.east_contract,
        }
    }

    pub fn contract_number_mut(&mut self, player: PlayerName) -> &mut ContractNumber {
        match player {
            PlayerName::User => &mut self.user_contract,
            PlayerName::West => &mut self.west_contract,
            PlayerName::North => &mut self.north_contract,
            PlayerName::East => &mut self.east_contract,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ContractPhaseAction {
    IncreaseBid,
    Pass,
    SetContractNumber(ContractNumber),
}
