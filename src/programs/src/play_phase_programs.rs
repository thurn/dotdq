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

use data::delegate_data::ProgramState;
use data::design::symbols;
use data::primitive::primitives::Suit;
use data::program_data::{ProgramDefinition, PROGRAMS};
use data::program_name::ProgramName;
use linkme::distributed_slice;
use ratatui::prelude::*;
use rules::rounds::tricks;

#[distributed_slice(PROGRAMS)]
pub fn starfall() -> ProgramDefinition {
    ProgramDefinition::new()
        .name(ProgramName::Starfall)
        .text(vec![Span::raw("↳Lead: Win the next trick.")])
        .play_phase(|on, id| {
            on.can_activate.this(id, |data, context| {
                context.state.is_none() && tricks::has_lead(data, context.id.owner)
            });
            on.activated.this(id, |data, context| {
                context.set_state(ProgramState::ActivatedForTrick(tricks::current_number(data)));
            });
            on.trick_winner.queried(id, |_, context, &number, current| {
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
        .play_phase(|on, id| {
            on.can_activate
                .this(id, |data, context| context.state.is_none() && data.is_turn(context.owner()));
            on.activated.this(id, |data, context| {
                data.trump = Some(Suit::Spades);
                context.set_state(ProgramState::Activated);
            });
        })
        .build()
}

#[distributed_slice(PROGRAMS)]
pub fn excavate() -> ProgramDefinition {
    ProgramDefinition::new()
        .name(ProgramName::Excavate)
        .text(vec![Span::raw("↳Round: You do not need to follow suit this trick.")])
        .play_phase(|on, id| {
            on.can_activate
                .this(id, |data, context| context.state.is_none() && data.is_turn(context.owner()));
            on.activated.this(id, |data, context| {
                context.set_state(ProgramState::ActivatedForTrick(tricks::current_number(data)));
            });
            on.must_follow_suit.queried(id, |_, context, &number, current| {
                if context.activated_for_trick(number) {
                    false
                } else {
                    current
                }
            })
        })
        .build()
}
