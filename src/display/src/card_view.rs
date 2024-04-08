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
use data::play_phase_data::PlayPhaseAction;
use data::primitives::{Card, Suit};
use data::widget_id::WidgetId;
use ratatui::layout::Offset;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use typed_builder::TypedBuilder;

use crate::render_context::RenderContext;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct CardView {
    card: Card,
    visible: bool,
    #[builder(default)]
    debug_visible: bool,
    #[builder(default)]
    on_click: Option<PlayPhaseAction>,
}

impl StatefulWidget for CardView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        Clear.render(area, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .border_style(Style::new().fg(if self.on_click.is_some() {
                "#dad45e".parse().unwrap()
            } else {
                Color::White
            }));
        let hovered =
            self.on_click.is_some() && context.hovered(WidgetId::CardView(self.card), area);
        let pressed =
            self.on_click.is_some() && context.mouse_down(WidgetId::CardView(self.card), area);
        if let Some(action) = self.on_click {
            context.clicked(
                WidgetId::CardView(self.card),
                area,
                GameAction::PlayPhaseAction(action),
            );
        }

        let inner = block.inner(area);
        block.render(area, buf);

        if self.visible || self.debug_visible {
            let text = if area.width <= 8 {
                let mut rank = self.card.rank.to_string();
                if area.width <= 6 {
                    rank = rank.replace("10", "T");
                };
                vec![
                    Line::from(text_style(rank, self.card, hovered, pressed)),
                    Line::from(text_style(self.card.suit.to_string(), self.card, hovered, pressed)),
                ]
            } else {
                vec![Line::from(text_style(self.card.to_string(), self.card, hovered, pressed))]
            };
            let [_, bottom] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(text.len() as u16)])
                .areas(inner);

            Paragraph::new(text.clone()).render(
                inner.offset(Offset { x: 0, y: if self.debug_visible { -1 } else { 0 } }),
                buf,
            );
            Paragraph::new(text).alignment(Alignment::Right).render(bottom, buf);
        }
    }
}

fn text_style<'a>(text: String, card: Card, hovered: bool, pressed: bool) -> Span<'a> {
    let color = match card.suit {
        Suit::Clubs => "#597dce".parse::<Color>().unwrap(),
        Suit::Diamonds => "#d2aa99".parse::<Color>().unwrap(),
        Suit::Hearts => "#d04648".parse::<Color>().unwrap(),
        Suit::Spades => "#6dc2ca".parse::<Color>().unwrap(),
    };

    let mut result = text.fg(color);
    result = if pressed { result.underlined() } else { result };

    if hovered {
        result.bg("#4e4a4e".parse().unwrap())
    } else {
        result
    }
}
