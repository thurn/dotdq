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
use ratatui::layout::{Alignment, Size};
use ratatui::prelude::Line;
use ratatui::widgets::StatefulWidget;
use rules::play_phase::play_phase_queries;

use crate::rendering::render_context::RenderContext;
use crate::rendering::widget_adapter::WidgetExt;
use crate::rounds::trick_view::TrickView;

pub trait PlayAreaDelegate {
    fn card_action(&self, player: PlayerName, card: Card) -> Option<GameAction>;

    fn is_card_visible(&self, player: PlayerName, card: Card) -> bool;

    fn status_bar(&self) -> impl StatefulWidget<State = RenderContext>;

    fn center_content(&self, card_size: Size) -> impl StatefulWidget<State = RenderContext>;
}

impl PlayAreaDelegate for PlayPhaseData {
    fn card_action(&self, player: PlayerName, card: Card) -> Option<GameAction> {
        if player == PlayerName::User
            && play_phase_queries::can_perform_action(self, player, PlayPhaseAction::PlayCard(card))
        {
            Some(GameAction::PlayPhaseAction(PlayPhaseAction::PlayCard(card)))
        } else {
            None
        }
    }

    fn is_card_visible(&self, _player: PlayerName, _card: Card) -> bool {
        true
    }

    fn status_bar(&self) -> impl StatefulWidget<State = RenderContext> {
        Line::from(format!(
            "User: {}/{} tricks",
            play_phase_queries::tricks_won(self, PlayerName::User),
            self.contracts.contract_number(PlayerName::User)
        ))
        .alignment(Alignment::Right)
        .adapt()
    }

    fn center_content(&self, card_size: Size) -> impl StatefulWidget<State = RenderContext> {
        TrickView::new().trick(self.current_trick.clone()).card_size(card_size).build()
    }
}
