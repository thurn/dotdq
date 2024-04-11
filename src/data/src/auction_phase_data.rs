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

use std::fmt::{Debug, Formatter};

use enumset::EnumSet;

use crate::bid_data::Bid;
use crate::primitives::PlayerName;

#[derive(Debug, Clone, Copy)]
pub struct AuctionPhaseData {
    /// Most recently bid Contract value, if any
    pub current: Option<Contract>,
    /// Bids which have been submitted by the user
    pub user_bids: EnumSet<Bid>,
    /// Bids which have been submitted by the opponent
    pub opponent_bids: EnumSet<Bid>,
}

/// A bid for a number of tricks a player has committed to winning with a given
/// trump suit
#[derive(Debug, Clone, Copy)]
pub struct Contract {
    /// Player who bid for this contract value
    pub declarer: PlayerName,
    /// Bid for this contract
    pub bid: Bid,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum AuctionPhaseAction {
    PlaceBid(Bid),
}

impl Debug for AuctionPhaseAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuctionPhaseAction::PlaceBid(bid) => {
                write!(f, "{:?}", bid)
            }
        }
    }
}
