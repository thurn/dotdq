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

use data::play_phase_data::{PlayPhaseData, PlayedCard};
use data::primitives::HandIdentifier;
use ratatui::layout::{Offset, Size};
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::card_view::CardView;
use crate::layout;
use crate::render_context::RenderContext;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct TrickView<'a> {
    data: &'a PlayPhaseData,
    card_size: Size,
}

impl<'a> TrickView<'a> {
    fn render_trick_card(
        &self,
        card: PlayedCard,
        parent_area: Rect,
        buf: &mut Buffer,
        context: &mut RenderContext,
    ) {
        let center = layout::centered_rect(self.card_size, parent_area);
        let target = match card.played_by {
            HandIdentifier::North => center
                .offset(Offset { x: 0, y: (self.card_size.height as f64 * -0.5).round() as i32 }),
            HandIdentifier::East => center
                .offset(Offset { x: (self.card_size.width as f64 * 0.5).round() as i32, y: 0 }),
            HandIdentifier::South => center
                .offset(Offset { x: 0, y: (self.card_size.height as f64 * 0.5).round() as i32 }),
            HandIdentifier::West => center
                .offset(Offset { x: (self.card_size.width as f64 * -0.5).round() as i32, y: 0 }),
        };
        CardView::new().card(card.card).visible(true).build().render(target, buf, context);
    }
}

impl<'a> StatefulWidget for TrickView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        for played in &self.data.current_trick.cards {
            self.render_trick_card(*played, area, buf, context);
        }
    }
}
