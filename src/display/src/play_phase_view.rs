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

use data::play_data::PlayPhaseData;
use data::primitives::HandIdentifier;
use ratatui::prelude::*;

use crate::render_context::RenderContext;
use crate::{horizontal_hand_view, vertical_hand_view};

pub fn render(data: &PlayPhaseData, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
    let [west, center, east] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .areas(area);

    let [north, _middle, south] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .areas(center);

    let card_size = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(12)])
        .split(south)[0]
        .as_size();

    horizontal_hand_view::render(data.hand(HandIdentifier::North), card_size, north, buf, context);
    vertical_hand_view::render(data.hand(HandIdentifier::East), card_size, east, buf, context);
    horizontal_hand_view::render(data.hand(HandIdentifier::South), card_size, south, buf, context);
    vertical_hand_view::render(data.hand(HandIdentifier::West), card_size, west, buf, context);
}
