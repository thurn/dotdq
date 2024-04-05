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

use data::primitives::{Card, Suit};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

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
        Clear.render(area, buf);
        let card = Block::default().borders(Borders::ALL).border_set(border::ROUNDED);
        if context.hovered(self.id(), area) {
            Block::default().on_blue().render(card.inner(area), buf);
        }
        let color = match self.card.suit {
            Suit::Clubs => "#597dce".parse::<Color>().unwrap(),
            Suit::Diamonds => "#d2aa99".parse::<Color>().unwrap(),
            Suit::Hearts => "#d04648".parse::<Color>().unwrap(),
            Suit::Spades => "#6dc2ca".parse::<Color>().unwrap(),
        };
        Paragraph::new(self.card.to_string().fg(color)).block(card).render(area, buf);
    }
}
