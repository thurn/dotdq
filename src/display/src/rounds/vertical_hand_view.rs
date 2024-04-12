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

use data::play_phase_data::{PlayPhaseAction, PlayPhaseData};
use data::primitives::PlayerName;
use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::{Offset, Rect, Size};
use ratatui::prelude::*;
use rules::play_phase::play_phase_queries;
use typed_builder::TypedBuilder;

use crate::rendering::layout;
use crate::rendering::render_context::RenderContext;
use crate::rounds::card_view::CardView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct VerticalHandView<'a> {
    data: &'a PlayPhaseData,
    player_name: PlayerName,
    card_size: Size,
}

impl<'a> StatefulWidget for VerticalHandView<'a> {
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

        for (i, card) in self.data.hands.hand(self.player_name).iter().sorted().enumerate() {
            let action = PlayPhaseAction::PlayCard(card);
            CardView::new()
                .card(card)
                .visible(false)
                .debug_visible(true)
                .on_click(
                    play_phase_queries::can_perform_action(self.data, PlayerName::User, action)
                        .then_some(action),
                )
                .build()
                .render(
                    card_rect.offset(Offset { x: 0, y: i as i32 * card_offset as i32 }),
                    buf,
                    context,
                );
        }
    }
}
