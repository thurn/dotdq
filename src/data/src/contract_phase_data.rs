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

use std::collections::HashMap;
use std::fmt::Debug;

use crate::delegate_data::{ContractPhaseDelegates, PlayPhaseDelegates};
use crate::game_action::GameAction;
use crate::play_phase_data::{Hands, PlayPhaseData, Trick};
use crate::primitives::{PlayerName, Suit};
use crate::program_data::ProgramData;

pub type ContractNumber = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ContractPhaseStep {
    AwaitingUserContact,
    AwaitingAgentContracts,
    ReadyToStart,
}

#[derive(Clone)]
pub struct ContractPhaseData {
    /// Trump suit to use for this round
    pub trump: Option<Suit>,
    /// Contract numbers which have currently been set, or 0 if no contract has
    /// yet been selected for a player.
    pub contracts: Contracts,
    /// Player hands
    pub hands: Hands,
    /// Current step within the contract phase
    pub step: ContractPhaseStep,
    /// Data about program for the players in this round
    pub programs: ProgramData<ContractPhaseDelegates>,
}

impl ContractPhaseData {
    pub fn to_play_phase(self) -> PlayPhaseData {
        let programs = ProgramData {
            current_delegates: PlayPhaseDelegates::default(),
            program_state: HashMap::new(),
            all_programs: self.programs.all_programs,
        };

        PlayPhaseData {
            current_trick: Trick::default(),
            completed_tricks: vec![],
            trump: self.trump,
            contracts: self.contracts,
            hands: self.hands,
            programs,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Contracts {
    user_contract: ContractNumber,
    west_contract: ContractNumber,
    north_contract: ContractNumber,
    east_contract: ContractNumber,
}

impl Contracts {
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
    SetUserContract(ContractNumber),
    AcceptUserContract,
    SetAgentContracts { west: ContractNumber, north: ContractNumber, east: ContractNumber },
    StartPlayPhase,
}

impl From<ContractPhaseAction> for GameAction {
    fn from(value: ContractPhaseAction) -> Self {
        GameAction::ContractAction(value)
    }
}
