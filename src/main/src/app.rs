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

use color_eyre::Result;
use crossterm::event;
use data::primitives::{Card, Rank, Suit};
use display::card_view::CardView;
use display::render_context::RenderContext;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph};
use rules::auction;

use crate::tui::Tui;

pub struct App;

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(tui: &mut Tui) -> Result<()> {
        let mut context = RenderContext::new(auction::new_game(&mut rand::thread_rng()));
        while !context.should_exit() {
            context.set_last_event(if event::poll(Duration::from_millis(16))? {
                Some(event::read()?)
            } else {
                None
            });
            tui.draw(|frame| frame.render_stateful_widget(App, frame.size(), &mut context))?;
        }
        Ok(())
    }

    // fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    //     match key_event.code {
    //         KeyCode::Char('q') => self.exit(),
    //         KeyCode::Left => self.decrement_counter()?,
    //         KeyCode::Right => self.increment_counter()?,
    //         _ => {}
    //     }
    //     Ok(())
    // }
}

impl StatefulWidget for App {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);
        CardView { card: Card::new(Suit::Diamonds, Rank::Queen) }.render(
            block.inner(area),
            buf,
            context,
        );
        let counter_text =
            Text::from(vec![Line::from(vec!["Value: ".into(), "12".to_string().yellow()])]);

        Paragraph::new(counter_text).centered().block(block).render(area, buf);
    }
}
