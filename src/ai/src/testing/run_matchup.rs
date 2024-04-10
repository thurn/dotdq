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

use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use data::primitives::PlayerName;
use rules::{auction, play_phase_queries};

use crate::core::agent::AgentConfig;
use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::game::agents;
use crate::game::agents::AgentName;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Verbosity {
    None,
    Matches,
    Actions,
}

#[derive(Parser)]
#[clap()]
pub struct Args {
    #[arg(value_enum)]
    pub user: AgentName,
    #[arg(value_enum)]
    pub opponent: AgentName,
    /// Maximum time in seconds for each agent to use for moves.
    #[arg(long, default_value_t = 1)]
    pub move_time: u64,
    /// Number of matches to run between these two named players
    #[arg(long, default_value_t = 1)]
    pub matches: u64,
    /// How much log output to produce while running
    #[arg(long, value_enum, default_value_t = Verbosity::Matches)]
    pub verbosity: Verbosity,
    /// Whether to crash the program if a search timeout is exceeded.
    #[arg(long, default_value_t = false)]
    pub panic_on_search_timeout: bool,
}

pub fn run(args: Args) {
    let user = agents::get_agent(args.user);
    let opponent = agents::get_agent(args.opponent);

    for i in 1..=args.matches {
        if args.verbosity >= Verbosity::Matches {
            println!(">>> Running match {} between {} and {}", i, user.name(), opponent.name());
        }
        let mut game = auction::new_game(&mut rand::thread_rng());
        if args.verbosity > Verbosity::None {
            println!("Starting game");
        }

        loop {
            match game.status() {
                GameStatus::InProgress { current_turn } => {
                    let agent = if current_turn == PlayerName::User { &user } else { &opponent };
                    let config = AgentConfig {
                        panic_on_search_timeout: args.panic_on_search_timeout,
                        deadline: Instant::now() + Duration::from_secs(args.move_time),
                    };
                    let action = agent.pick_action(config, &game);
                    game.execute_action(current_turn, action);
                    clear_action_line(args.verbosity);
                    if args.verbosity > Verbosity::None {
                        println!("{} performs action {:?}", agent.name(), action);
                    }
                }
                GameStatus::Completed { winner } => {
                    let agent = if winner == PlayerName::User { &user } else { &opponent };
                    if args.verbosity >= Verbosity::Matches {
                        clear_action_line(args.verbosity);
                        println!(
                            "{} wins, {} to {}",
                            agent.name(),
                            play_phase_queries::tricks_won(&game, winner),
                            play_phase_queries::tricks_won(&game, winner.opponent())
                        );
                    }
                    break;
                }
            }
        }
    }
}

fn clear_action_line(verbosity: Verbosity) {
    if verbosity == Verbosity::Matches {
        print!("\x1B[1F"); // Moves cursor to beginning of previous line, 1 line up
        print!("\x1B[2K"); // Erase the entire line
    }
}
