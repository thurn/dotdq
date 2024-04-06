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

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use data::game_action::GameAction;
use data::widget_id::WidgetId;
use ratatui::layout::Position;
use ratatui::prelude::*;

#[derive(Default)]
pub struct RenderContext {
    event: Option<Event>,
    current_hover: Option<WidgetId>,
    _last_mouse_down: Option<WidgetId>,
    exit: bool,
    action: Option<GameAction>,
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

    pub fn set_current_hover(&mut self, current: Option<WidgetId>) {
        self.current_hover = current;
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn pop_render(&mut self) -> Option<GameAction> {
        let action = self.action;
        self.action = None;
        self.event = None;
        action
    }

    pub fn hovered(&mut self, id: WidgetId, area: Rect) -> bool {
        let current = self.current_hover == Some(id);
        let Some(Event::Mouse(e)) = self.event else {
            return current;
        };

        if e.kind == MouseEventKind::Moved {
            if area.contains(Position::new(e.column, e.row)) {
                self.action = Some(GameAction::SetHover(id));
            } else if current {
                self.action = Some(GameAction::ClearHover);
            }
        }

        current
    }

    pub fn mouse_down(&self, area: Rect) -> bool {
        matches!(self.event, Some(Event::Mouse(e)) if e.kind == MouseEventKind::Down(MouseButton::Left)
                    && area.contains(Position::new(e.column, e.row)))
    }

    pub fn clicked(&self, area: Rect) -> bool {
        matches!(self.event, Some(Event::Mouse(e)) if e.kind == MouseEventKind::Up(MouseButton::Left)
                    && area.contains(Position::new(e.column, e.row)))
    }
}
