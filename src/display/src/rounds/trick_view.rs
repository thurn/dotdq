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

use data::play_phase_data::{PlayedCard, Trick};
use data::primitive::primitives::PlayerName;
use ratatui::layout::{Offset, Size};
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::core::layout;
use crate::core::render_context::RenderContext;
use crate::rounds::card_view::CardView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct TrickView {
    trick: Trick,
    card_size: Size,
}

impl TrickView {
    fn render_trick_card(
        &self,
        card: PlayedCard,
        parent_area: Rect,
        buf: &mut Buffer,
        context: &mut RenderContext,
    ) {
        let center = layout::centered_rect(self.card_size, parent_area);
        let offset = 0.65;
        let target = match card.played_by {
            PlayerName::North => center.offset(Offset {
                x: 0,
                y: (self.card_size.height as f64 * -offset).round() as i32,
            }),
            PlayerName::East => center
                .offset(Offset { x: (self.card_size.width as f64 * offset).round() as i32, y: 0 }),
            PlayerName::User => center
                .offset(Offset { x: 0, y: (self.card_size.height as f64 * offset).round() as i32 }),
            PlayerName::West => center
                .offset(Offset { x: (self.card_size.width as f64 * -offset).round() as i32, y: 0 }),
        };
        CardView::new().card(card.card).visible(true).build().render(target, buf, context);
    }
}

impl StatefulWidget for TrickView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        for played in &self.trick.cards {
            self.render_trick_card(*played, area, buf, context);
        }
    }
}
