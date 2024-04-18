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
use crate::primitive::primitives::PlayerName;
use crate::program_name::ProgramName;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ActivationState {
    CannotActivate,
    CanActivate,
    CurrentlyActive,
    PreviouslyActivated,
}

impl ActivationState {
    pub fn can_activate(&self) -> bool {
        *self == Self::CanActivate
    }
}

pub trait HasPrograms {
    fn get_state(&self, id: &ProgramId) -> Option<ProgramState>;

    fn set_state(&mut self, id: ProgramId, state: Option<ProgramState>);

    fn activation_state(&self, program: ProgramId) -> ActivationState;

    fn activate(&mut self, program: ProgramId);
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ProgramState {
    ActivatedForTrick(TrickNumber),
    Activated,
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

    pub fn owner(&self) -> PlayerName {
        self.id.owner
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
    current: Vec<QueryFn<TData, TArg, TResult>>,
    delegates: Vec<(ProgramId, QueryFn<TData, TArg, TResult>)>,
}

impl<TData: HasPrograms, TArg, TResult> QueryDelegateList<TData, TArg, TResult> {
    pub fn queried(&mut self, value: QueryFn<TData, TArg, TResult>) {
        self.current.push(value);
    }

    pub fn set_current_id(&mut self, id: ProgramId) {
        for function in self.current.drain(..) {
            self.delegates.push((id, function));
        }
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
        Self { current: vec![], delegates: vec![] }
    }
}

#[derive(Clone)]
pub struct ProgramQuery<TData: HasPrograms, TResult> {
    current: Option<SingleQueryFn<TData, TResult>>,
    delegates: HashMap<ProgramId, SingleQueryFn<TData, TResult>>,
}

impl<TData: HasPrograms, TResult> Default for ProgramQuery<TData, TResult> {
    fn default() -> Self {
        Self { current: None, delegates: HashMap::default() }
    }
}

impl<TData: HasPrograms, TResult> ProgramQuery<TData, TResult> {
    pub fn this(&mut self, value: SingleQueryFn<TData, TResult>) {
        self.current = Some(value);
    }

    pub fn set_current_id(&mut self, id: ProgramId) {
        assert!(!self.delegates.contains_key(&id), "Delegate already registered for {id:?}");
        if let Some(function) = self.current.take() {
            self.delegates.insert(id, function);
        }
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
    current: Option<SingleMutationFn<TData>>,
    delegates: HashMap<ProgramId, SingleMutationFn<TData>>,
}

impl<TData: HasPrograms> Default for ProgramMutation<TData> {
    fn default() -> Self {
        Self { current: None, delegates: HashMap::default() }
    }
}

impl<TData: HasPrograms> ProgramMutation<TData> {
    pub fn this(&mut self, value: SingleMutationFn<TData>) {
        self.current = Some(value);
    }

    pub fn set_current_id(&mut self, id: ProgramId) {
        assert!(!self.delegates.contains_key(&id), "Delegate already registered for {id:?}");
        if let Some(function) = self.current.take() {
            self.delegates.insert(id, function);
        }
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

#[derive(Clone)]
pub struct PlayerTrickNumber {
    pub player_name: PlayerName,
    pub trick_number: TrickNumber,
}

impl PlayerTrickNumber {
    pub fn new(player_name: PlayerName, trick_number: TrickNumber) -> Self {
        Self { player_name, trick_number }
    }
}

#[derive(Default, Clone)]
pub struct PlayPhaseDelegates {
    pub can_activate: ProgramQuery<PlayPhaseData, bool>,
    pub currently_active: ProgramQuery<PlayPhaseData, bool>,
    pub activated: ProgramMutation<PlayPhaseData>,
    pub trick_winner: QueryDelegateList<PlayPhaseData, TrickNumber, PlayerName>,
    pub must_follow_suit: QueryDelegateList<PlayPhaseData, PlayerTrickNumber, bool>,
}

impl PlayPhaseDelegates {
    pub fn set_current_id(&mut self, id: ProgramId) {
        self.can_activate.set_current_id(id);
        self.currently_active.set_current_id(id);
        self.activated.set_current_id(id);
        self.trick_winner.set_current_id(id);
        self.must_follow_suit.set_current_id(id);
    }
}

#[derive(Default)]
pub struct RunPhaseDelegates {}
