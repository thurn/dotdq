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
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use rules::play_phase::play_phase_queries;
use typed_builder::TypedBuilder;

use crate::core::colors;
use crate::core::render_context::RenderContext;
use crate::rounds::play_area_delegate::PlayAreaDelegate;
use crate::rounds::play_area_view::PlayAreaView;
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
    }

    fn center_content(&self, card_size: Size) -> impl StatefulWidget<State = RenderContext> {
        TrickView::new().trick(self.current_trick.clone()).card_size(card_size).build()
    }
}

fn contract_string(data: &PlayPhaseData, name: PlayerName) -> Line {
    Line::from(
        format!(
            "{name}: {}/{}",
            play_phase_queries::tricks_won(data, name),
            data.contracts.contract_number(name)
        )
        .fg(colors::white()),
    )
}
