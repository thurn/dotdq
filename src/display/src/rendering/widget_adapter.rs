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

use ratatui::prelude::*;

use crate::rendering::render_context::RenderContext;

pub trait WidgetExt<T: Widget> {
    fn adapt(self) -> WidgetAdapter<T>;
}

impl<T: Widget> WidgetExt<T> for T {
    fn adapt(self) -> WidgetAdapter<T> {
        WidgetAdapter::new(self)
    }
}

pub struct WidgetAdapter<T: Widget> {
    widget: T,
}

impl<T: Widget> WidgetAdapter<T> {
    pub fn new(widget: T) -> Self {
        Self { widget }
    }
}

impl<T: Widget> StatefulWidget for WidgetAdapter<T> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, _: &mut Self::State) {
        self.widget.render(area, buf)
    }
}
