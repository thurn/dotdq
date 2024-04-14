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

use data::contract_phase_data::{ContractPhaseData, ContractPhaseStep};
use data::game_action::GameAction;
use data::primitives::{Card, PlayerName};
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::core::colors;
use crate::core::render_context::RenderContext;
use crate::rounds::contract_view::ContractView;
use crate::rounds::play_area_delegate::PlayAreaDelegate;
use crate::rounds::play_area_view::PlayAreaView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ContractPhaseView<'a> {
    data: &'a ContractPhaseData,
}

impl<'a> StatefulWidget for ContractPhaseView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        PlayAreaView::new()
            .delegate(self.data)
            .hands(&self.data.hands)
            .build()
            .render(area, buf, context)
    }
}

impl PlayAreaDelegate for ContractPhaseData {
    fn card_action(&self, _player: PlayerName, _card: Card) -> Option<GameAction> {
        None
    }

    fn is_card_visible(&self, _player: PlayerName, _card: Card) -> bool {
        true
    }

    fn render_top_status_bar(&self, area: Rect, buf: &mut Buffer, _: &mut RenderContext) {
        if self.step == ContractPhaseStep::ReadyToStart {
            contract_string(self, PlayerName::West).alignment(Alignment::Left).render(area, buf);
            contract_string(self, PlayerName::North).alignment(Alignment::Center).render(area, buf);
            contract_string(self, PlayerName::East).alignment(Alignment::Right).render(area, buf);
        }
    }

    fn render_bottom_status_bar(&self, area: Rect, buf: &mut Buffer, _: &mut RenderContext) {
        if self.step == ContractPhaseStep::ReadyToStart {
            contract_string(self, PlayerName::User).alignment(Alignment::Center).render(area, buf);
        }
    }

    fn center_content(&self, _card_size: Size) -> impl StatefulWidget<State = RenderContext> {
        ContractView::new().data(self).build()
    }
}

pub fn contract_string(data: &ContractPhaseData, name: PlayerName) -> Line {
    Line::from(
        format!("{name}: {} Tricks", data.contracts.contract_number(name)).fg(colors::white()),
    )
}
