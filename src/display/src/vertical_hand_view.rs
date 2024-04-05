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
use ratatui::buffer::Buffer;
use ratatui::layout::{Offset, Rect, Size};

use crate::render_context::RenderContext;
use crate::{card_view, layout};

pub fn render(
    hand: impl Iterator<Item = Card>,
    card_size: Size,
    area: Rect,
    buf: &mut Buffer,
    context: &mut RenderContext,
) {
    let card_offset = 1;
    let count = 13;
    let target = layout::centered_rect(
        Size::new(card_size.width, count as u16 * card_offset + card_size.height),
        area,
    );
    let card_rect =
        Rect { x: target.x, y: target.y, width: card_size.width, height: card_size.height };

    for (i, card) in hand.enumerate() {
        card_view::render(
            card,
            false,
            card_rect.offset(Offset { x: 0, y: i as i32 * card_offset as i32 }),
            buf,
            context,
        );
    }
}
