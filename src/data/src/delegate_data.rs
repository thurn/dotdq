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

use crate::play_phase_data::{PlayPhaseData, TrickNumber};
use crate::primitives::PlayerName;
use crate::program_name::ProgramName;

pub trait HasProgramState {
    fn get_state(&self, id: &ProgramId) -> Option<ProgramState>;
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ProgramState {
    ActivatedForTrick(TrickNumber),
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ProgramId {
    pub name: ProgramName,
    pub owner: PlayerName,
}

impl ProgramId {
    pub fn new(name: ProgramName, owner: PlayerName) -> Self {
        Self { name, owner }
    }
}

#[derive(Clone, Copy)]
pub struct Context {
    pub id: ProgramId,
    pub state: Option<ProgramState>,
}

impl Context {
    pub fn set_state(&mut self, state: ProgramState) {
        self.state = Some(state);
    }

    pub fn activated_for_trick(&self, trick_number: TrickNumber) -> bool {
        matches!(self.state, Some(ProgramState::ActivatedForTrick(t)) if t == trick_number)
    }
}

pub type QueryFn<TData, TArg, TResult> = fn(&TData, &Context, &TArg, TResult) -> TResult;

#[derive(Clone)]
pub struct QueryDelegateList<TData: HasProgramState, TArg, TResult> {
    delegates: Vec<(ProgramId, QueryFn<TData, TArg, TResult>)>,
}

impl<TData: HasProgramState, TArg, TResult> QueryDelegateList<TData, TArg, TResult> {
    pub fn queried(&mut self, id: ProgramId, value: QueryFn<TData, TArg, TResult>) {
        self.delegates.push((id, value));
    }

    pub fn run_query(&self, data: &TData, arg: &TArg, current: TResult) -> TResult {
        let mut result = current;
        for (program_id, function) in &self.delegates {
            let context = Context { id: *program_id, state: data.get_state(program_id) };
            result = function(data, &context, arg, result);
        }
        result
    }
}

impl<TData: HasProgramState, TContext, TResult> Default
    for QueryDelegateList<TData, TContext, TResult>
{
    fn default() -> Self {
        Self { delegates: vec![] }
    }
}

#[derive(Clone)]
pub struct SingleDelegateMap<T> {
    delegates: HashMap<ProgramId, T>,
}

impl<T> Default for SingleDelegateMap<T> {
    fn default() -> Self {
        Self { delegates: HashMap::default() }
    }
}

impl<T> SingleDelegateMap<T> {
    pub fn this(&mut self, id: ProgramId, value: T) {
        self.delegates.insert(id, value);
    }
}

#[derive(Clone)]
pub struct EventDelegateList<T>(Vec<T>);

impl<T> EventDelegateList<T> {
    pub fn on_event(&mut self, value: T) {
        self.0.push(value);
    }
}

impl<T> Default for EventDelegateList<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

#[derive(Default, Clone)]
pub struct ContractPhaseDelegates {}

#[derive(Default, Clone)]
pub struct PlayPhaseDelegates {
    pub can_activate: SingleDelegateMap<fn(&PlayPhaseData, &Context) -> bool>,
    pub activated: SingleDelegateMap<fn(&mut PlayPhaseData, &mut Context)>,
    pub trick_winner: QueryDelegateList<PlayPhaseData, TrickNumber, PlayerName>,
}

#[derive(Default)]
pub struct RunPhaseDelegates {}
