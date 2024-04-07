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

use std::time::Duration;

use ai::ai_agent_action;
use color_eyre::Result;
use crossterm::event;
use data::game_action::GameAction;
use data::play_phase_data::PlayPhaseData;
use data::primitives::PlayerName;
use display::play_phase_view::PlayPhaseView;
use display::render_context::RenderContext;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};
use rules::{auction, play_phase_actions, play_phase_queries};
use tracing::info;

use crate::tui::Tui;

pub struct App<'a> {
    pub data: &'a PlayPhaseData,
}

impl<'a> App<'a> {
    /// runs the application's main loop until the user quits
    pub fn run(tui: &mut Tui) -> Result<()> {
        let mut data = auction::new_game(&mut rand::thread_rng());
        let mut context = RenderContext::default();
        while !context.should_exit() {
            context.set_last_event(if event::poll(Duration::from_millis(16))? {
                Some(event::read()?)
            } else {
                None
            });
            tui.draw(|frame| loop {
                frame.render_stateful_widget(App { data: &data }, frame.size(), &mut context);
                let Some(action) = context.finish_render() else {
                    break;
                };
                info!(?action, "Got action");
                match action {
                    GameAction::PlayPhaseAction(a) => {
                        info!(?a, "Handling PlayPhaseAction");
                        play_phase_actions::handle_action(&mut data, a);
                        while play_phase_queries::current_turn(&data) == Some(PlayerName::Opponent)
                        {
                            let ai_action = ai_agent_action::select(&data);
                            play_phase_actions::handle_action(&mut data, ai_action);
                        }
                    }
                    GameAction::SetHover(id) => {
                        context.set_current_hover(id);
                    }
                    GameAction::SetMouseDown(id) => {
                        context.set_current_mouse_down(id);
                    }
                };
            })?;
        }
        Ok(())
    }
}

impl<'a> StatefulWidget for App<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        if area.width < 80 || area.height < 24 {
            Paragraph::new(vec![
                Line::from(
                    "Error: The minimum terminal size for this game is 80 columns by 24 rows!",
                ),
                Line::from(format!("Your terminal is {} by {}.", area.width, area.height)),
                Line::from("Press 'q' to quit."),
            ])
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .render(area, buf);
        } else {
            PlayPhaseView::new().data(self.data).build().render(area, buf, context);
        }
    }
}
