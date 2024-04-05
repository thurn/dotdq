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
use ratatui::layout::Flex;
use ratatui::prelude::*;

use crate::card_view::CardView;
use crate::render_context::RenderContext;

pub struct HorizontalHandView<'a> {
    pub hand: &'a HashSet<Card>,
}

impl<'a> StatefulWidget for HorizontalHandView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(iter::repeat(Constraint::Length(5)).take(self.hand.len()))
            .flex(Flex::Center)
            .split(area)
            .iter()
            .zip(self.hand.iter().copied().sorted())
            .for_each(|(&rect, card)| CardView { card }.render(rect, buf, context));
    }
}
