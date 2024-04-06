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

use data::play_phase_data::PlayPhaseData;
use data::primitives::HandIdentifier;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::horizontal_hand_view::HorizontalHandView;
use crate::render_context::RenderContext;
use crate::trick_view::TrickView;
use crate::vertical_hand_view::VerticalHandView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct PlayPhaseView<'a> {
    data: &'a PlayPhaseData,
}

impl<'a> StatefulWidget for PlayPhaseView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [west, center, east] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .areas(area);

        let [north, tricks, south] = Layout::default()
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

        HorizontalHandView::new()
            .data(self.data)
            .hand(HandIdentifier::North)
            .card_size(card_size)
            .build()
            .render(north, buf, context);
        VerticalHandView::new()
            .hand(self.data.hand(HandIdentifier::East))
            .card_size(card_size)
            .build()
            .render(east, buf, context);
        HorizontalHandView::new()
            .data(self.data)
            .hand(HandIdentifier::South)
            .card_size(card_size)
            .build()
            .render(south, buf, context);
        VerticalHandView::new()
            .hand(self.data.hand(HandIdentifier::West))
            .card_size(card_size)
            .build()
            .render(west, buf, context);

        TrickView::new().data(self.data).card_size(card_size).build().render(tricks, buf, context);
    }
}
