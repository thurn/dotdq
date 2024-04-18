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

use linkme::distributed_slice;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::delegate_data::{
    ContractPhaseDelegates, PlayPhaseDelegates, ProgramId, ProgramState, RunPhaseDelegates,
};
use crate::primitive::primitives::PlayerName;
use crate::program_name::ProgramName;

#[distributed_slice]
pub static PROGRAMS: [fn() -> ProgramDefinition];

#[derive(Clone)]
pub struct ProgramData<T> {
    pub current_delegates: T,
    pub program_state: HashMap<ProgramId, ProgramState>,
    pub all_programs: HashMap<PlayerName, Vec<ProgramName>>,

    /// Programs which have been activated in the current scope.
    pub activated: HashSet<ProgramId>,
}

impl<T> ProgramData<T> {
    pub fn for_player(&self, player: PlayerName) -> impl Iterator<Item = ProgramName> + '_ {
        self.all_programs.get(&player).into_iter().flatten().copied()
    }
}

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ProgramDefinition {
    pub name: ProgramName,
    pub text: Vec<Span<'static>>,
    #[builder(default, setter(strip_option))]
    pub contract_phase: Option<fn(&mut ContractPhaseDelegates)>,
    #[builder(default, setter(strip_option))]
    pub play_phase: Option<fn(&mut PlayPhaseDelegates)>,
    #[builder(default, setter(strip_option))]
    pub run_phase: Option<fn(&mut RunPhaseDelegates)>,
}
