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

use data::game_action::GameAction;
use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitives::{Card, PlayerName};
use ratatui::prelude::*;
use rules::play_phase::play_phase_queries;
use typed_builder::TypedBuilder;

use crate::rendering::render_context::RenderContext;
use crate::rounds::horizontal_hand_view::HorizontalHandView;
use crate::rounds::trick_view::TrickView;
use crate::rounds::vertical_hand_view::VerticalHandView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct PlayAreaView<'a> {
    data: &'a PlayPhaseData,
}

impl<'a> PlayAreaView<'a> {
    fn card_action(&self) -> impl Fn(Card) -> Option<GameAction> + '_ {
        |card| {
            play_phase_queries::can_perform_action(
                self.data,
                PlayerName::North,
                PlayPhaseAction::PlayCard(card),
            )
            .then_some(GameAction::PlayPhaseAction(PlayPhaseAction::PlayCard(card)))
        }
    }
}

impl<'a> StatefulWidget for PlayAreaView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [status_bar, card_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .areas(area);
        Line::from(format!(
            "User: {}/{} tricks",
            play_phase_queries::tricks_won(self.data, PlayerName::User),
            self.data.contracts.contract_number(PlayerName::User)
        ))
        .alignment(Alignment::Right)
        .render(status_bar, buf);

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
            .hand(self.data.hands.hand(PlayerName::North))
            .card_size(card_size)
            .card_action(self.card_action())
            .build()
            .render(north, buf, context);
        VerticalHandView::new()
            .hand(self.data.hands.hand(PlayerName::East))
            .card_size(card_size)
            .build()
            .render(east, buf, context);
        HorizontalHandView::new()
            .hand(self.data.hands.hand(PlayerName::User))
            .card_size(card_size)
            .card_action(self.card_action())
            .build()
            .render(north, buf, context);
        VerticalHandView::new()
            .hand(self.data.hands.hand(PlayerName::West))
            .card_size(card_size)
            .build()
            .render(west, buf, context);

        TrickView::new().data(self.data).card_size(card_size).build().render(tricks, buf, context);
    }
}
