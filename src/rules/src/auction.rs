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

use std::slice::ChunksExact;

use data::auction_phase_data::Contract;
use data::bid_data::Bid;
use data::play_phase_data::PlayPhaseData;
use data::primitives::{Card, PlayerName, Rank, Suit};
use enumset::EnumSet;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn new_game(rng: &mut impl Rng) -> PlayPhaseData {
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

    PlayPhaseData::new(
        Contract { declarer: PlayerName::User, bid: Bid::SixClubs },
        north,
        east,
        south,
        west,
    )
}

fn build_hand(chunks: &mut ChunksExact<Card>) -> EnumSet<Card> {
    EnumSet::from_iter(chunks.next().expect("Invalid deck size").iter().copied())
}
