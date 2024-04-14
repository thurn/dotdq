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

use data::contract_phase_data::ContractPhaseData;
use data::game_action::GameAction;
use data::primitives::{Card, PlayerName};
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use ratatui::widgets::Clear;
use typed_builder::TypedBuilder;

use crate::core::render_context::RenderContext;
use crate::core::widget_adapter::WidgetExt;
use crate::rounds::contract_bid_view::ContractBidView;
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

    fn status_bar(&self) -> impl StatefulWidget<State = RenderContext> {
        Clear.adapt()
    }

    fn center_content(&self, _card_size: Size) -> impl StatefulWidget<State = RenderContext> {
        ContractBidView::new().data(self).build()
    }
}
