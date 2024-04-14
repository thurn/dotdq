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
use data::primitives::{Card, PlayerName};
use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::StatefulWidget;

use crate::core::render_context::RenderContext;

pub trait PlayAreaDelegate {
    fn card_action(&self, player: PlayerName, card: Card) -> Option<GameAction>;

    fn is_card_visible(&self, player: PlayerName, card: Card) -> bool;

    fn render_top_status_bar(&self, area: Rect, buf: &mut Buffer, context: &mut RenderContext);

    fn render_bottom_status_bar(&self, area: Rect, buf: &mut Buffer, context: &mut RenderContext);

    fn center_content(&self, card_size: Size) -> impl StatefulWidget<State = RenderContext>;
}
