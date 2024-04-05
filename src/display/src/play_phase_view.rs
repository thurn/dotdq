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

use std::collections::HashSet;

use data::play_data::PlayPhaseData;
use data::primitives::HandIdentifier;
use ratatui::prelude::*;

use crate::horizontal_hand_view::HorizontalHandView;
use crate::render_context::RenderContext;

pub struct PlayPhaseView<'a> {
    pub data: &'a PlayPhaseData,
}

impl<'a> StatefulWidget for PlayPhaseView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [_west, center, _east] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .areas(area);

        let [_north, _middle, south] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .areas(center);

        HorizontalHandView { hand: &self.data.hand(HandIdentifier::South).collect::<HashSet<_>>() }
            .render(south, buf, context)
    }
}
