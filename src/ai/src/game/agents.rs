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

use std::marker::PhantomData;

use data::play_phase_data::PlayPhaseData;

use crate::core::agent::{Agent, AgentData};
use crate::game::evaluators::TrickEvaluator;
use crate::monte_carlo::monte_carlo_search::{MonteCarloAlgorithm, RandomPlayoutEvaluator};
use crate::monte_carlo::uct1::Uct1;
use crate::tree_search::alpha_beta::AlphaBetaAlgorithm;

pub enum AgentName {
    AlphaBeta,
    Uct1,
}

pub fn get_agent(name: AgentName) -> Box<dyn Agent<PlayPhaseData>> {
    match name {
        AgentName::AlphaBeta => Box::new(ALPHA_BETA_AGENT),
        AgentName::Uct1 => Box::new(UCT1_AGENT),
    }
}

const ALPHA_BETA_AGENT: AgentData<AlphaBetaAlgorithm, TrickEvaluator, PlayPhaseData> =
    AgentData::omniscient("ALPHA_BETA", AlphaBetaAlgorithm { search_depth: 10 }, TrickEvaluator);

pub const UCT1_AGENT: AgentData<
    MonteCarloAlgorithm<Uct1>,
    RandomPlayoutEvaluator<PlayPhaseData, TrickEvaluator>,
    PlayPhaseData,
> = AgentData::omniscient(
    "UCT1",
    MonteCarloAlgorithm { child_score_algorithm: Uct1 {} },
    RandomPlayoutEvaluator { evaluator: TrickEvaluator, phantom_data: PhantomData },
);
