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

use data::design::symbols;
use data::primitive::primitives::Suit;
use data::program_data::{ProgramDefinition, PROGRAMS};
use data::program_name::ProgramName;
use linkme::distributed_slice;
use ratatui::prelude::*;
use rules::program::activation;
use rules::program::activation::DuringTurn;

#[distributed_slice(PROGRAMS)]
pub fn starfall() -> ProgramDefinition {
    ProgramDefinition::new()
        .name(ProgramName::Starfall)
        .text(vec![Span::raw("↳Round: Win this trick.")])
        .play_phase(|on| {
            activation::activate_for_trick::<DuringTurn>(on);
            on.trick_winner.queried(|_, context, &number, current| {
                if context.activated_for_trick(number) {
                    context.id.owner
                } else {
                    current
                }
            })
        })
        .build()
}

#[distributed_slice(PROGRAMS)]
pub fn obsidian() -> ProgramDefinition {
    ProgramDefinition::new()
        .name(ProgramName::Obsidian)
        .text(vec![Span::raw("↳Round: Change the trump suit to "), symbols::suit(Suit::Spades)])
        .play_phase(|on| {
            activation::can_activate::<DuringTurn>(on);
            on.activated.this(|data, _| {
                data.trump = Some(Suit::Spades);
            });
        })
        .build()
}

#[distributed_slice(PROGRAMS)]
pub fn eviction() -> ProgramDefinition {
    ProgramDefinition::new()
        .name(ProgramName::Eviction)
        .text(vec![Span::raw("↳Round: You do not need to follow suit this trick.")])
        .play_phase(|on| {
            activation::activate_for_trick::<DuringTurn>(on);
            on.must_follow_suit.queried(|_, context, p, current| {
                if p.player_name == context.owner() && context.activated_for_trick(p.trick_number) {
                    false
                } else {
                    current
                }
            })
        })
        .build()
}
