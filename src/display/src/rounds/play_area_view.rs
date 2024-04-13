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

use data::play_phase_data::Hands;
use data::primitives::PlayerName;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::rendering::render_context::RenderContext;
use crate::rounds::horizontal_hand_view::HorizontalHandView;
use crate::rounds::play_area_delegate::PlayAreaDelegate;
use crate::rounds::vertical_hand_view::VerticalHandView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct PlayAreaView<'a, TDelegate>
where
    TDelegate: PlayAreaDelegate,
{
    delegate: &'a TDelegate,
    hands: &'a Hands,
}

impl<'a, TDelegate> StatefulWidget for PlayAreaView<'a, TDelegate>
where
    TDelegate: PlayAreaDelegate,
{
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [status_bar, card_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .areas(area);
        self.delegate.status_bar().render(status_bar, buf, context);

        let [west, center, east] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .areas(card_area);

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
            .hand(self.hands.hand(PlayerName::North))
            .card_size(card_size)
            .player_name(PlayerName::North)
            .delegate(self.delegate)
            .build()
            .render(north, buf, context);
        VerticalHandView::new()
            .hand(self.hands.hand(PlayerName::East))
            .card_size(card_size)
            .build()
            .render(east, buf, context);
        HorizontalHandView::new()
            .hand(self.hands.hand(PlayerName::User))
            .card_size(card_size)
            .player_name(PlayerName::User)
            .delegate(self.delegate)
            .build()
            .render(south, buf, context);
        VerticalHandView::new()
            .hand(self.hands.hand(PlayerName::West))
            .card_size(card_size)
            .build()
            .render(west, buf, context);

        self.delegate.center_content(card_size).render(tricks, buf, context);
    }
}
