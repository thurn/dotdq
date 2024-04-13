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

use data::play_phase_data::PlayPhaseData;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::rendering::render_context::RenderContext;
use crate::rounds::play_area_view::PlayAreaView;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct PlayPhaseView<'a> {
    data: &'a PlayPhaseData,
}

impl<'a> StatefulWidget for PlayPhaseView<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        PlayAreaView::new()
            .delegate(self.data)
            .hands(&self.data.hands)
            .build()
            .render(area, buf, context)
    }
}
