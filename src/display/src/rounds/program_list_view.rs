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

use data::delegate_data::{ActivationState, HasPrograms, ProgramId};
use data::design::colors;
use data::play_phase_data::PlayPhaseAction;
use data::widget_id::WidgetId;
use ratatui::prelude::*;
use typed_builder::TypedBuilder;

use crate::core::render_context::RenderContext;

pub const WIDTH: u16 = 8;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ProgramListView<'a, T: HasPrograms> {
    data: &'a T,
    programs: Vec<ProgramId>,
}

impl<'a, T: HasPrograms> StatefulWidget for ProgramListView<'a, T> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let split = Layout::vertical(
            iter::once(Constraint::Fill(1))
                .chain(self.programs.iter().map(|_| Constraint::Length(1))),
        )
        .split(area);

        for (i, &program) in self.programs.iter().enumerate() {
            ProgramNameView::new()
                .id(program)
                .activation(self.data.activation_state(program))
                .build()
                .render(split[i + 1], buf, context);
        }
    }
}

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct ProgramNameView {
    id: ProgramId,
    activation: ActivationState,
}

impl StatefulWidget for ProgramNameView {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let widget_id = WidgetId::Program(self.id);
        let hovered = self.activation.can_activate() && context.hovered(widget_id, area);
        let pressed = self.activation.can_activate() && context.mouse_down(widget_id, area);
        if self.activation.can_activate() {
            context.clicked(widget_id, area, PlayPhaseAction::ActivateProgram(self.id));
        }

        let mut style = match self.activation {
            ActivationState::CannotActivate => Style::new().fg(colors::white()).bold(),
            ActivationState::CanActivate => {
                Style::new().fg(colors::can_activate()).bold().underlined()
            }
            ActivationState::CurrentlyActive => Style::new().fg(colors::light_blue()).bold(),
            ActivationState::PreviouslyActivated => Style::new().fg(colors::white()).crossed_out(),
        };

        if pressed {
            style = style.remove_modifier(Modifier::UNDERLINED);
        }

        if hovered {
            style = style.bg(colors::selected());
        }

        Line::styled(self.id.name.to_string(), style).render(area, buf);
    }
}
