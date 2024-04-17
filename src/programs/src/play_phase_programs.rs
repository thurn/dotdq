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
use data::design::colors;
use data::program_name::ProgramName;
use data::programs::{ProgramDefinition, PROGRAMS};
use linkme::distributed_slice;
use rules::rounds::tricks;

pub fn linkme() {}

#[distributed_slice(PROGRAMS)]
pub fn redstar() -> ProgramDefinition {
    ProgramDefinition::new()
        .symbol('★')
        .color(colors::red())
        .name(ProgramName::Redstar)
        .text("↳Lead: Win this trick.")
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
