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

use std::iter;

use data::delegate_data::ProgramId;
use data::design::colors;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::core::render_context::RenderContext;

pub const WIDTH: u16 = 8;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ProgramListView {
    programs: Vec<ProgramId>,
}

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ProgramNameView {
    id: ProgramId,
}

impl StatefulWidget for ProgramListView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let split = Layout::vertical(
            iter::once(Constraint::Fill(1))
                .chain(self.programs.iter().map(|_| Constraint::Length(1))),
        )
        .split(area);

        for (i, &program) in self.programs.iter().enumerate() {
            ProgramNameView::new().id(program).build().render(split[i + 1], buf, context);
        }
    }
}

impl StatefulWidget for ProgramNameView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, _: &mut RenderContext) {
        Line::styled(
            self.id.name.to_string(),
            Style::new().fg(colors::white()).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )
        .render(area, buf);
    }
}
