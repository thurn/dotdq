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

use std::collections::HashSet;
use std::iter;

use data::primitives::Card;
use itertools::Itertools;
use ratatui::layout::{Flex, Offset};
use ratatui::prelude::*;

use crate::card_view::CardView;
use crate::render_context::RenderContext;

pub struct HorizontalHandView<'a> {
    pub hand: &'a HashSet<Card>,
}

impl<'a> StatefulWidget for HorizontalHandView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [card_rect] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(12)])
            .areas(area);
        let suits = self.hand.iter().copied().sorted().group_by(|card| card.suit);

        let mut offset = 0;
        for (_, group) in &suits {
            for card in group {
                CardView { card }.render(
                    card_rect.offset(Offset { x: offset, y: 0 }),
                    buf,
                    context,
                );
                offset += (card_rect.width as f32 * 0.4).round() as i32;
            }

            offset += i32::from(card_rect.width);
        }
    }
}

pub struct HorizontalSuitView<'a> {
    pub cards: &'a Vec<Card>,
}

impl<'a> StatefulWidget for HorizontalSuitView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let [card_rect] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40)])
            .areas(area);
        for (i, card) in self.cards.iter().enumerate() {
            CardView { card: *card }.render(
                card_rect.offset(Offset { x: (i * 3) as i32, y: 0 }),
                buf,
                context,
            )
        }
    }
}
