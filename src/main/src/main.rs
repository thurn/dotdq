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

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use programs::play_phase_programs;
use tracing::info;

pub mod app;
pub mod cli;
pub mod tui;
pub mod utils;

fn main() -> Result<()> {
    utils::initialize_logging()?;
    utils::initialize_panic_handler()?;
    Cli::parse();

    // Required to make the linker not discard program under OSX, see
    // https://github.com/dtolnay/linkme/issues/61
    play_phase_programs::linkme();

    let mut tui = tui::enter()?;
    let commit = env!("VERGEN_GIT_SHA");
    info!(commit, "Starting game");
    app::run(&mut tui)?;
    tui::exit()?;
    Ok(())
}
