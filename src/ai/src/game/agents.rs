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

use clap::ValueEnum;
use data::play_phase_data::PlayPhaseData;

use crate::core::agent::{Agent, AgentData};
use crate::core::first_available_action::FirstAvailableActionAlgorithm;
use crate::core::win_loss_evaluator::WinLossEvaluator;
use crate::game::evaluators::{MaxTricksEvaluator, TrickEvaluator};
use crate::monte_carlo::monte_carlo_search::{MonteCarloAlgorithm, RandomPlayoutEvaluator};
use crate::monte_carlo::uct1::Uct1;
use crate::tree_search::alpha_beta::AlphaBetaAlgorithm;

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AgentName {
    AlphaBetaDepth10,
    AlphaBetaDepth13,
    Uct1,
    Uct1MaxTricks,
    Uct1Iterations250,
    FirstAvailableAction,
}

pub fn get_agent(name: AgentName) -> Box<dyn Agent<PlayPhaseData>> {
    match name {
        AgentName::AlphaBetaDepth10 => Box::new(AgentData::omniscient(
            "ALPHA_BETA_10",
            AlphaBetaAlgorithm { search_depth: 10 },
            TrickEvaluator,
        )),
        AgentName::AlphaBetaDepth13 => Box::new(AgentData::omniscient(
            "ALPHA_BETA_13",
            AlphaBetaAlgorithm { search_depth: 13 },
            TrickEvaluator,
        )),
        AgentName::Uct1 => Box::new(AgentData::omniscient(
            "UCT1",
            MonteCarloAlgorithm { child_score_algorithm: Uct1 {}, max_iterations: None },
            RandomPlayoutEvaluator { evaluator: TrickEvaluator, phantom_data: PhantomData },
        )),
        AgentName::Uct1MaxTricks => Box::new(AgentData::omniscient(
            "UCT1_MAX_TRICKS",
            MonteCarloAlgorithm { child_score_algorithm: Uct1 {}, max_iterations: None },
            RandomPlayoutEvaluator { evaluator: MaxTricksEvaluator, phantom_data: PhantomData },
        )),
        AgentName::Uct1Iterations250 => Box::new(AgentData::omniscient(
            "UCT1_250",
            MonteCarloAlgorithm { child_score_algorithm: Uct1 {}, max_iterations: Some(250) },
            RandomPlayoutEvaluator { evaluator: TrickEvaluator, phantom_data: PhantomData },
        )),
        AgentName::FirstAvailableAction => Box::new(AgentData::omniscient(
            "FIRST_AVAILABLE_ACTION",
            FirstAvailableActionAlgorithm,
            WinLossEvaluator,
        )),
    }
}
