//!
//! ReqMD Explorer
//!

use crate::widgets::{Explorer, ExplorerState};
use ::color_eyre::Result;
use ::crossterm::event::{self, Event};
use ::ratatui::DefaultTerminal;
use ::reqmd_http as http;
use ::reqmd_markdown as markdown;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct App {
    should_halt: bool,
    explorer_state: ExplorerState,
    requests: Vec<http::Request>,
}

impl App {
    pub fn new(requests: Vec<http::Request>) -> Self {
        Self {
            should_halt: false,
            requests,
            explorer_state: ExplorerState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<Option<&http::Request>> {
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

        Ok(self
            .explorer_state
            .selected()
            .and_then(|idx| self.requests.get(idx)))
    }

    fn process_event(&mut self, terminal_event: Event) {
        match terminal_event {
            Event::Key(key)
                if key.kind.is_press() && (key.code.is_char('q') || key.code.is_esc()) =>
            {
                self.explorer_state.unselect();
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
            Event::Key(key) if key.kind.is_press() && key.code.is_enter() => {
                self.should_halt = true;
            }
            _ => {}
        }
    }
}

impl From<markdown::ast::Document> for App {
    fn from(doc: markdown::ast::Document) -> Self {
        let factory = doc.meta.http.factory();
        let requests = doc
            .requests
            .into_iter()
            .map(|req| {
                factory
                    .builder()
                    .method(req.method)
                    .path(req.path)
                    .multiple_query_params(req.query)
                    .multiple_headers(req.headers)
                    .body(req.body.content)
                    .build()
            })
            .collect();
        Self::new(requests)
    }
}

pub mod widgets;
