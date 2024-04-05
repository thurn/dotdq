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
use itertools::Itertools;
use ratatui::layout::Offset;
use ratatui::prelude::*;

use crate::card_view;
use crate::render_context::RenderContext;

pub fn render(
    hand: impl Iterator<Item = Card>,
    area: Rect,
    buf: &mut Buffer,
    context: &mut RenderContext,
) {
    let [card_rect] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(12)])
        .areas(area);
    let suits = hand.sorted().group_by(|card| card.suit);

    let mut offset = 0;
    for (_, group) in &suits {
        for card in group {
            card_view::render(card, card_rect.offset(Offset { x: offset, y: 0 }), buf, context);
            offset += (card_rect.width as f32 * 0.4).round() as i32;
        }

        offset += i32::from(card_rect.width);
    }
}
