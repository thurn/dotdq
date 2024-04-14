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

use data::contract_phase_data::{ContractPhaseAction, ContractPhaseData};
use data::primitives::PlayerName;
use data::widget_id::WidgetId;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget};
use typed_builder::TypedBuilder;

use crate::core::button::{Button, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::core::render_context::RenderContext;
use crate::core::{colors, layout};

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ContractBidView<'a> {
    data: &'a ContractPhaseData,
}

impl<'a> StatefulWidget for ContractBidView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let center = layout::centered_rect(Size { width: 50, height: 14 }, area);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN)
            .border_style(colors::white());

        let [top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(block.inner(center));

        block.render(center, buf);

        let trump = Line::from(
            format!("Trump Suit: {}", self.data.trump.map_or("NT".to_string(), |s| s.to_string()))
                .fg(colors::trump(self.data.trump)),
        );
        let bid = Line::from(
            format!("Current Contract: {}", self.data.contracts.contract_number(PlayerName::User))
                .fg(colors::white()),
        );

        Paragraph::new(vec![trump, bid]).alignment(Alignment::Center).render(top, buf);

        let buttons = layout::centered_rect(
            Size { width: 2 + (2 * BUTTON_WIDTH), height: BUTTON_HEIGHT },
            bottom,
        );
        let [left, _, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Length(2), Constraint::Fill(1)])
            .areas(buttons);

        Button::new()
            .label("Increase\nContract")
            .action(ContractPhaseAction::IncreaseContract)
            .id(WidgetId::IncreaseContractButton)
            .build()
            .render(left, buf, context);
        Button::new()
            .label("Accept\nContract")
            .action(ContractPhaseAction::AcceptContract)
            .id(WidgetId::AcceptContractButton)
            .build()
            .render(right, buf, context);
    }
}
