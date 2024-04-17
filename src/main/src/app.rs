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
use data::contract_phase_data::ContractPhaseStep;
use data::game_action::GameAction;
use data::primitive::primitives::PlayerName;
use data::round_data::RoundData;
use display::core::render_context::RenderContext;
use display::rounds::contract_phase_view::ContractPhaseView;
use display::rounds::play_phase_view::PlayPhaseView;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};
use rules::contract_phase::contract_phase_actions;
use rules::play_phase::{play_phase_actions, play_phase_queries};
use rules::rounds::new_round;
use tracing::info;

use crate::tui::Tui;

pub fn run(tui: &mut Tui) -> Result<()> {
    let mut data = new_round::create(&mut rand::thread_rng());
    let mut context = RenderContext::default();
    let mut ai_search_running = false;
    while !context.should_exit() {
        context.set_last_event(if event::poll(Duration::from_millis(16))? {
            Some(event::read()?)
        } else {
            None
        });
        tui.draw(|frame| loop {
            frame.render_stateful_widget(App { data: &data }, frame.size(), &mut context);

            let action = if let Some(action) = context.finish_render() {
                action
            } else if let Some(action) = ai_agent_action::poll_action() {
                ai_search_running = false;
                action
            } else {
                break;
            };
            match (&mut data, action) {
                (RoundData::PlayPhase(play_data), GameAction::PlayAction(a)) => {
                    info!(?a, "Handling PlayPhaseAction");
                    let Some(current_player) = play_phase_queries::current_turn(play_data) else {
                        break;
                    };
                    play_phase_actions::handle_action(play_data, current_player, a);
                    let Some(next_player) = play_phase_queries::current_turn(play_data) else {
                        continue;
                    };
                    if next_player.is_agent() && !ai_search_running {
                        ai_search_running = true;
                        ai_agent_action::initiate_selection(play_data.clone());
                    }
                }
                (RoundData::ContractPhase(contract_data), GameAction::ContractAction(a)) => {
                    info!(?a, "Handling ContractPhaseAction");
                    let result =
                        contract_phase_actions::handle_action(contract_data, PlayerName::User, a);
                    if contract_data.step == ContractPhaseStep::AwaitingAgentContracts
                        && !ai_search_running
                    {
                        ai_search_running = true;
                        ai_agent_action::populate_agent_contracts(contract_data.clone());
                    }
                    if let Some(r) = result {
                        data = r;
                    }
                }
                (_, GameAction::SetHover(id)) => {
                    context.set_current_hover(id);
                }
                (_, GameAction::SetMouseDown(id)) => {
                    context.set_current_mouse_down(id);
                }
                _ => {
                    panic!("Action {:?} not valid for current phase", action);
                }
            };
        })?;
    }
    Ok(())
}

pub struct App<'a> {
    pub data: &'a RoundData,
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
            match self.data {
                RoundData::ContractPhase(c) => {
                    ContractPhaseView::new().data(c).build().render(area, buf, context);
                }
                RoundData::PlayPhase(p) => {
                    PlayPhaseView::new().data(p).build().render(area, buf, context);
                }
            }
        }
    }
}
