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

use std::cmp;

use data::contract_phase_data::{ContractPhaseAction, ContractPhaseData, ContractPhaseStep};
use data::primitives::PlayerName;
use data::widget_id::WidgetId;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget};
use typed_builder::TypedBuilder;

use crate::core::button::Button;
use crate::core::render_context::RenderContext;
use crate::core::{colors, layout};

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ContractView<'a> {
    data: &'a ContractPhaseData,
}

impl<'a> StatefulWidget for ContractView<'a> {
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

        let contract_number = self.data.contracts.contract_number(PlayerName::User);
        let mut lines = vec![Line::from(
            format!("Trump Suit: {}", self.data.trump.map_or("NT".to_string(), |s| s.to_string()))
                .fg(colors::trump(self.data.trump)),
        )];
        if self.data.step != ContractPhaseStep::ReadyToStart {
            lines.push(Line::from(
                format!("Current Contract: {}", contract_number).fg(colors::white()),
            ));
        }

        Paragraph::new(lines).alignment(Alignment::Center).render(top, buf);

        let bottom_display = layout::centered_rect(Size { width: 50, height: 4 }, bottom);

        match self.data.step {
            ContractPhaseStep::AwaitingUserContact => {
                let [_, left, _, middle, _, right, _] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                    ])
                    .areas(bottom_display);

                Button::new()
                    .label("Increase\nContract")
                    .action(ContractPhaseAction::SetUserContract(cmp::min(13, contract_number + 1)))
                    .id(WidgetId::IncreaseContractButton)
                    .build()
                    .render(left, buf, context);
                Button::new()
                    .label("Decrease\nContract")
                    .action(ContractPhaseAction::SetUserContract(contract_number.saturating_sub(1)))
                    .id(WidgetId::DecreaseContractButton)
                    .build()
                    .render(middle, buf, context);
                Button::new()
                    .label("Accept\nContract")
                    .action(ContractPhaseAction::AcceptUserContract)
                    .id(WidgetId::AcceptContractButton)
                    .build()
                    .render(right, buf, context);
            }
            ContractPhaseStep::AwaitingAgentContracts => {
                Paragraph::new("Awaiting agents...")
                    .fg(colors::white())
                    .alignment(Alignment::Center)
                    .render(bottom_display, buf);
            }
            ContractPhaseStep::ReadyToStart => {
                let ready = layout::centered_rect(Size { width: 25, height: 4 }, bottom_display);
                Button::new()
                    .label("Start Round")
                    .action(ContractPhaseAction::StartPlayPhase)
                    .id(WidgetId::DecreaseContractButton)
                    .build()
                    .render(ready, buf, context);
            }
        }
    }
}
