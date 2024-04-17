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

pub trait HasPrograms {
    fn get_state(&self, id: &ProgramId) -> Option<ProgramState>;

    fn set_state(&mut self, id: ProgramId, state: Option<ProgramState>);

    fn can_activate(&self, program: ProgramId) -> bool;

    fn activate(&mut self, program: ProgramId);
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ProgramState {
    ActivatedForTrick(TrickNumber),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
    pub fn new(id: ProgramId, state: Option<ProgramState>) -> Self {
        Self { id, state }
    }

    pub fn set_state(&mut self, state: ProgramState) {
        self.state = Some(state);
    }

    pub fn activated_for_trick(&self, trick_number: TrickNumber) -> bool {
        matches!(self.state, Some(ProgramState::ActivatedForTrick(t)) if t == trick_number)
    }
}

pub type SingleQueryFn<TData, TResult> = fn(&TData, &Context) -> TResult;
pub type QueryFn<TData, TArg, TResult> = fn(&TData, &Context, &TArg, TResult) -> TResult;
pub type SingleMutationFn<TData> = fn(&mut TData, &mut Context);
pub type MutationFn<TData, TArg> = fn(&mut TData, &mut Context, &TArg);

#[derive(Clone)]
pub struct QueryDelegateList<TData: HasPrograms, TArg, TResult> {
    delegates: Vec<(ProgramId, QueryFn<TData, TArg, TResult>)>,
}

impl<TData: HasPrograms, TArg, TResult> QueryDelegateList<TData, TArg, TResult> {
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

impl<TData: HasPrograms, TContext, TResult> Default
    for QueryDelegateList<TData, TContext, TResult>
{
    fn default() -> Self {
        Self { delegates: vec![] }
    }
}

#[derive(Clone)]
pub struct ProgramQuery<TData: HasPrograms, TResult> {
    delegates: HashMap<ProgramId, SingleQueryFn<TData, TResult>>,
}

impl<TData: HasPrograms, TResult> Default for ProgramQuery<TData, TResult> {
    fn default() -> Self {
        Self { delegates: HashMap::default() }
    }
}

impl<TData: HasPrograms, TResult> ProgramQuery<TData, TResult> {
    pub fn this(&mut self, id: ProgramId, value: SingleQueryFn<TData, TResult>) {
        self.delegates.insert(id, value);
    }

    pub fn run_query(&self, data: &TData, program_id: ProgramId, current: TResult) -> TResult {
        let Some(function) = self.delegates.get(&program_id) else {
            return current;
        };
        let context = Context { id: program_id, state: data.get_state(&program_id) };
        function(data, &context)
    }
}

#[derive(Clone)]
pub struct ProgramMutation<TData: HasPrograms> {
    delegates: HashMap<ProgramId, SingleMutationFn<TData>>,
}

impl<TData: HasPrograms> Default for ProgramMutation<TData> {
    fn default() -> Self {
        Self { delegates: HashMap::default() }
    }
}

impl<TData: HasPrograms> ProgramMutation<TData> {
    pub fn this(&mut self, id: ProgramId, value: SingleMutationFn<TData>) {
        self.delegates.insert(id, value);
    }

    pub fn get_mutation_fn(&mut self, program_id: ProgramId) -> SingleMutationFn<TData> {
        *self
            .delegates
            .get(&program_id)
            .unwrap_or_else(|| panic!("Program not found {program_id:?}"))
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
    pub can_activate: ProgramQuery<PlayPhaseData, bool>,
    pub activated: ProgramMutation<PlayPhaseData>,
    pub trick_winner: QueryDelegateList<PlayPhaseData, TrickNumber, PlayerName>,
}

#[derive(Default)]
pub struct RunPhaseDelegates {}
