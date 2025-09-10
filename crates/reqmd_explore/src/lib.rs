//!
//! ReqMD Explorer
//!

use crate::widgets::{Explorer, ExplorerState};
use ::color_eyre::Result;
use ::crossterm::event::{self, Event};
use ::ratatui::DefaultTerminal;
use ::reqmd_core::HttpGroup;
use ::reqmd_http as http;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct App {
    should_halt: bool,
    explorer_state: ExplorerState,
    requests: Vec<http::Request>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        Explorer::new(&self.requests).next(&mut self.explorer_state);
        while !self.should_halt {
            terminal.draw(|frame| {
                let explorer = Explorer::new(&self.requests);
                frame.render_stateful_widget(&explorer, frame.area(), &mut self.explorer_state);
            })?;

            if event::poll(Duration::from_millis(500))? {
                let event = event::read()?;
                self.process_event(event);
            }
        }
        Ok(())
    }

    fn process_event(&mut self, terminal_event: Event) {
        match terminal_event {
            Event::Key(key) if key.kind.is_press() && key.code.is_char('q') => {
                self.should_halt = true;
            }
            Event::Key(key) if key.kind.is_press() && key.code.is_up() => {
                Explorer::new(&self.requests).prev(&mut self.explorer_state);
            }
            Event::Key(key) if key.kind.is_press() && key.code.is_char('k') => {
                Explorer::new(&self.requests).prev(&mut self.explorer_state);
            }
            Event::Key(key) if key.kind.is_press() && key.code.is_down() => {
                Explorer::new(&self.requests).next(&mut self.explorer_state);
            }
            Event::Key(key) if key.kind.is_press() && key.code.is_char('j') => {
                Explorer::new(&self.requests).next(&mut self.explorer_state);
            }
            _ => {}
        }
    }
}

impl From<HttpGroup> for App {
    fn from(group: HttpGroup) -> Self {
        Self {
            should_halt: false,
            requests: group.iter_requests().map(|(_, req)| req).collect(),
            explorer_state: ExplorerState::default(),
        }
    }
}

#[cfg(test)]
pub mod support;
pub mod widgets;
