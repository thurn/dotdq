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

use crate::horizontal_hand_view;
use crate::render_context::RenderContext;

pub fn render(data: &PlayPhaseData, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
    let [_west, center, _east] = Layout::default()
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

    horizontal_hand_view::render(data.hand(HandIdentifier::North), north, buf, context);
    horizontal_hand_view::render(data.hand(HandIdentifier::South), south, buf, context);
}
