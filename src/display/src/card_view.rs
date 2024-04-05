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

use data::primitives::Card;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::render_context::RenderContext;
use crate::widget_id::WidgetId;

pub struct CardView {
    pub card: Card,
}

impl CardView {
    pub fn id(&self) -> WidgetId {
        WidgetId::CardView(self.card)
    }
}

impl StatefulWidget for CardView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let center = centered_rect(9, 6, area);
        let card = Block::default().borders(Borders::ALL).border_set(border::ROUNDED);
        if context.hovered(self.id(), center) {
            Block::default().on_blue().render(card.inner(center), buf);
        }
        Paragraph::new("Q♦".red()).block(card).render(center, buf);
    }
}

/// helper function to create a centered rect using up certain percentage of the
/// available rect `r`
fn centered_rect(length_x: u16, length_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(length_y), Constraint::Fill(1)])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(length_x), Constraint::Fill(1)])
        .split(popup_layout[1])[1] // Return the middle chunk
}
