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

use data::delegate_data::{Context, PlayPhaseDelegates, ProgramState};
use data::play_phase_data::PlayPhaseData;

use crate::rounds::tricks;

pub trait CanActivate {
    fn can_activate(data: &PlayPhaseData, context: &Context) -> bool;
}

pub struct DuringTurn;
impl CanActivate for DuringTurn {
    fn can_activate(data: &PlayPhaseData, context: &Context) -> bool {
        data.is_turn(context.owner())
    }
}

pub struct WithLead;
impl CanActivate for WithLead {
    fn can_activate(data: &PlayPhaseData, context: &Context) -> bool {
        tricks::has_lead(data, context.owner())
    }
}

pub fn can_activate<TActivate: CanActivate>(on: &mut PlayPhaseDelegates) {
    on.can_activate.this(|data, context| TActivate::can_activate(data, context));
}

pub fn activate_for_trick<TActivate: CanActivate>(on: &mut PlayPhaseDelegates) {
    can_activate::<TActivate>(on);
    on.currently_active
        .this(|data, context| context.activated_for_trick(tricks::current_number(data)));
    on.activated.this(|data, context| {
        context.set_state(ProgramState::ActivatedForTrick(tricks::current_number(data)));
    });
}
