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

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::layout::Position;
use ratatui::prelude::*;

use crate::widget_id::WidgetId;

#[derive(Default)]
pub struct RenderContext {
    event: Option<Event>,
    last_hover: Option<WidgetId>,
    _last_mouse_down: Option<WidgetId>,
    exit: bool,
}

impl RenderContext {
    pub fn set_last_event(&mut self, event: Option<Event>) {
        if let Some(Event::Key(e)) = event {
            if e.kind == KeyEventKind::Press && e.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }

        self.event = event;
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn hovered(&mut self, id: WidgetId, area: Rect) -> bool {
        if let Some(Event::Mouse(e)) = self.event {
            if e.kind == MouseEventKind::Moved {
                if area.contains(Position::new(e.column, e.row)) {
                    self.last_hover = Some(id);
                    return true;
                } else if self.last_hover == Some(id) {
                    // Mouse move event not over this element, clear hover state
                    self.last_hover = None;
                }
            }
        }

        self.last_hover == Some(id)
    }
}
