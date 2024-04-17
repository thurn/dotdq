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
use std::slice::ChunksExact;

use data::contract_phase_data::{ContractPhaseData, ContractPhaseStep, Contracts};
use data::delegate_data::PlayPhaseDelegates;
use data::play_phase_data::{Hands, PlayPhaseData, Trick};
use data::primitives::{Card, Rank, Suit};
use data::round_data::RoundData;
use enumset::EnumSet;
use rand::prelude::SliceRandom;
use rand::Rng;

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
    })
}

pub fn create_play_phase(rng: &mut impl Rng) -> PlayPhaseData {
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
    PlayPhaseData {
        current_trick: Trick::default(),
        completed_tricks: vec![],
        trump,
        contracts: Contracts::default(),
        hands: Hands::new(north, east, south, west),
        delegates: PlayPhaseDelegates::default(),
        program_state: HashMap::new(),
    }
}

fn build_hand(chunks: &mut ChunksExact<Card>) -> EnumSet<Card> {
    EnumSet::from_iter(chunks.next().expect("Invalid deck size").iter().copied())
}
