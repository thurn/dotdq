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

use data::delegate_data::ProgramId;
use data::design::colors;
use data::game_action::GameAction;
use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitives::{Card, PlayerName};
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use rules::play_phase::play_phase_queries;
use rules::rounds::tricks;
use typed_builder::TypedBuilder;

use crate::core::render_context::RenderContext;
use crate::rounds::play_area_delegate::PlayAreaDelegate;
use crate::rounds::play_area_view::PlayAreaView;
use crate::rounds::program_list_view;
use crate::rounds::program_list_view::ProgramListView;
use crate::rounds::trick_view::TrickView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct PlayPhaseView<'a> {
    data: &'a PlayPhaseData,
}

impl<'a> StatefulWidget for PlayPhaseView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        PlayAreaView::new()
            .delegate(self.data)
            .hands(&self.data.hands)
            .build()
            .render(area, buf, context)
    }
}

impl PlayAreaDelegate for PlayPhaseData {
    fn card_action(&self, player: PlayerName, card: Card) -> Option<GameAction> {
        if player == PlayerName::User
            && play_phase_queries::can_perform_action(self, player, PlayPhaseAction::PlayCard(card))
        {
            Some(GameAction::PlayAction(PlayPhaseAction::PlayCard(card)))
        } else {
            None
        }
    }

    fn is_card_visible(&self, _player: PlayerName, _card: Card) -> bool {
        true
    }

    fn render_top_status_bar(&self, area: Rect, buf: &mut Buffer, _: &mut RenderContext) {
        contract_string(self, PlayerName::West).alignment(Alignment::Left).render(area, buf);
        contract_string(self, PlayerName::North).alignment(Alignment::Center).render(area, buf);
        contract_string(self, PlayerName::East).alignment(Alignment::Right).render(area, buf);
    }

    fn render_bottom_status_bar(&self, area: Rect, buf: &mut Buffer, _: &mut RenderContext) {
        contract_string(self, PlayerName::User).alignment(Alignment::Center).render(area, buf);
        Line::from(
            format!("Trump: {}", self.trump.map_or("NT".to_string(), |s| s.to_string()))
                .fg(colors::trump(self.trump)),
        )
        .alignment(Alignment::Right)
        .render(area, buf);
    }

    fn render_center_content(
        &self,
        card_size: Size,
        area: Rect,
        buf: &mut Buffer,
        context: &mut RenderContext,
    ) {
        let [program_list, tricks, _] = Layout::horizontal([
            Constraint::Length(program_list_view::WIDTH),
            Constraint::Fill(1),
            Constraint::Length(program_list_view::WIDTH),
        ])
        .areas(area);
        let programs = self
            .programs
            .all_programs
            .get(&PlayerName::User)
            .unwrap_or(&vec![])
            .iter()
            .map(|&name| ProgramId::new(name, PlayerName::User))
            .collect();
        ProgramListView::new().programs(programs).build().render(program_list, buf, context);

        let trick = if self.current_trick.is_started() {
            Some(self.current_trick.clone())
        } else {
            self.completed_tricks.last().map(|t| t.trick.clone())
        };

        if let Some(t) = trick {
            TrickView::new().trick(t).card_size(card_size).build().render(tricks, buf, context)
        }
    }
}

fn contract_string(data: &PlayPhaseData, name: PlayerName) -> Line {
    Line::from(
        format!("{name}: {}/{}", tricks::won(data, name), data.contracts.contract_number(name))
            .fg(colors::white()),
    )
}
