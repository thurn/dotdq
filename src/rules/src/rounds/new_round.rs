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
use std::slice::ChunksExact;

use data::contract_phase_data::{ContractNumber, ContractPhaseData, ContractPhaseStep, Contracts};
use data::delegate_data::ContractPhaseDelegates;
use data::play_phase_data::{Hands, PlayPhaseData};
use data::primitive::primitives::{Card, PlayerName, Rank, Suit};
use data::program_data::ProgramData;
use data::program_name::ProgramName;
use data::round_data::RoundData;
use enumset::EnumSet;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::rounds::start_play_phase;

pub fn create(rng: &mut impl Rng) -> RoundData {
    let mut cards = Vec::new();
    for suit in enum_iterator::all::<Suit>() {
        for rank in enum_iterator::all::<Rank>() {
            cards.push(Card::new(suit, rank))
        }
    }
    cards.shuffle(rng);

    let mut chunks = cards.chunks_exact(13);
    let north = build_hand(&mut chunks);
    let east = build_hand(&mut chunks);
    let south = build_hand(&mut chunks);
    let west = build_hand(&mut chunks);

    let trump =
        *[None, Some(Suit::Clubs), Some(Suit::Diamonds), Some(Suit::Hearts), Some(Suit::Spades)]
            .choose(rng)
            .expect("Empty slice");
    RoundData::ContractPhase(ContractPhaseData {
        trump,
        contracts: Contracts::default(),
        hands: Hands::new(north, east, south, west),
        step: ContractPhaseStep::AwaitingUserContact,
        programs: ProgramData {
            current_delegates: ContractPhaseDelegates::default(),
            program_state: HashMap::default(),
            all_programs: HashMap::from([(PlayerName::User, vec![
                ProgramName::Starfall,
                ProgramName::Obsidian,
                ProgramName::Eviction,
            ])]),
            activated: HashSet::new(),
        },
    })
}

pub fn create_play_phase(rng: &mut impl Rng, contract: ContractNumber) -> PlayPhaseData {
    let RoundData::ContractPhase(mut data) = create(rng) else {
        panic!("Expected ContractPhase");
    };
    *data.contracts.contract_number_mut(PlayerName::User) = contract;
    *data.contracts.contract_number_mut(PlayerName::West) = contract;
    *data.contracts.contract_number_mut(PlayerName::North) = contract;
    *data.contracts.contract_number_mut(PlayerName::East) = contract;

    start_play_phase::run(data)
}

fn build_hand(chunks: &mut ChunksExact<Card>) -> EnumSet<Card> {
    EnumSet::from_iter(chunks.next().expect("Invalid deck size").iter().copied())
}
