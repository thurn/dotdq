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
use ratatui::layout::{Offset, Size};
use ratatui::prelude::*;
use rules::play_phase::play_phase_queries;
use typed_builder::TypedBuilder;

use crate::rendering::render_context::RenderContext;
use crate::rounds::card_view::CardView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct HorizontalHandView<'a> {
    data: &'a PlayPhaseData,
    player_name: PlayerName,
    card_size: Size,
}

impl<'a> StatefulWidget for HorizontalHandView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let card_offset = (self.card_size.width as f32 * 0.4).round();
        let card_rect = Rect {
            x: area.x,
            y: area.y,
            width: self.card_size.width,
            height: self.card_size.height,
        };
        let suits =
            self.data.hands.hand(self.player_name).iter().sorted().group_by(|card| card.suit());

        let mut offset = 0;
        for (_, group) in &suits {
            for card in group {
                let action = PlayPhaseAction::PlayCard(card);
                CardView::new()
                    .card(card)
                    .visible(true)
                    .on_click(
                        play_phase_queries::can_perform_action(self.data, PlayerName::User, action)
                            .then_some(action),
                    )
                    .build()
                    .render(card_rect.offset(Offset { x: offset, y: 0 }), buf, context);
                offset += card_offset as i32;
            }

            offset += i32::from(card_rect.width);
        }
    }
}
