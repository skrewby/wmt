pub mod client_table;
pub mod workspace_table;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

use crate::screen::{table_screen::TableScreen, Screen, ScreenEvent};

pub struct App {
    exit: bool,
    screens: Vec<Box<dyn Screen>>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn new() -> Result<App> {
        let table_screen = TableScreen::new().context("Creating table screen")?;
        let mut screens: Vec<Box<dyn Screen>> = Vec::new();
        screens.push(Box::new(table_screen));
        Ok(App {
            exit: false,
            screens,
        })
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),

            _ => {
                if let Some(widget) = self.screens.last_mut() {
                    if let Some(screen_event) = widget.handle_key_event(key_event) {
                        self.handle_screen_event(screen_event);
                    }
                }
            }
        }
    }

    fn handle_screen_event(&mut self, screen_event: ScreenEvent) {
        match screen_event {
            ScreenEvent::Close => self.exit = true,
            ScreenEvent::AddScreen(screen) => {
                self.screens.push(screen);
            }
            ScreenEvent::PopScreen => {
                if let None = self.screens.pop() {
                    self.exit = true;
                }
            }
            ScreenEvent::PopAndRefresh => {
                if let None = self.screens.pop() {
                    self.exit = true;
                }
                if let Some(widget) = self.screens.last_mut() {
                    widget.refresh();
                }
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(widget) = self.screens.last() {
            widget.render_ref(area, buf);
        }
    }
}
