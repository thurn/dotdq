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

use std::time::Duration;

use ai::game::agents::AgentName;
use ai::testing::run_matchup;
use ai::testing::run_matchup::{Args, Verbosity};
use criterion::{criterion_group, criterion_main, Criterion};

criterion_main!(benches);
criterion_group!(benches, alpha_beta, uct1);

pub fn alpha_beta(c: &mut Criterion) {
    let mut group = c.benchmark_group("alpha_beta");
    group.measurement_time(Duration::from_secs(60));
    group.bench_function("Alpha Beta", |b| {
        b.iter(|| {
            run_matchup::run(Args {
                user: AgentName::AlphaBetaDepth10,
                opponent: AgentName::AlphaBetaDepth10,
                move_time: 1,
                matches: 1,
                verbosity: Verbosity::None,
                panic_on_search_timeout: false,
            })
        })
    });
}

pub fn uct1(c: &mut Criterion) {
    let mut group = c.benchmark_group("uct1");
    group.measurement_time(Duration::from_secs(30));
    group.bench_function("UCT1", |b| {
        b.iter(|| {
            run_matchup::run(Args {
                user: AgentName::Uct1Iterations250,
                opponent: AgentName::Uct1Iterations250,
                move_time: 1,
                matches: 1,
                verbosity: Verbosity::None,
                panic_on_search_timeout: false,
            })
        })
    });
}
