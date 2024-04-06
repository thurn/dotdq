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
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::card_view::CardView;
use crate::layout;
use crate::render_context::RenderContext;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct VerticalHandView<TIterator>
where
    TIterator: Iterator<Item = Card>,
{
    hand: TIterator,
    card_size: Size,
}

impl<TIterator: Iterator<Item = Card>> StatefulWidget for VerticalHandView<TIterator> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let card_offset = 1;
        let count = 13;
        let target = layout::centered_rect(
            Size::new(self.card_size.width, count as u16 * card_offset + self.card_size.height),
            area,
        );
        let card_rect = Rect {
            x: target.x,
            y: target.y,
            width: self.card_size.width,
            height: self.card_size.height,
        };

        for (i, card) in self.hand.enumerate() {
            CardView::new().card(card).visible(true).build().render(
                card_rect.offset(Offset { x: 0, y: i as i32 * card_offset as i32 }),
                buf,
                context,
            );
        }
    }
}
