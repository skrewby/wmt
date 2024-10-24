mod client_table;
mod workspace_table;

use std::io;

use anyhow::{Context, Result};
use client_table::ClientTable;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};
use workspace_table::WorkspaceTable;

use crate::hypr::Hypr;

enum SelectedScreen {
    Clients,
    Workspaces,
}

pub struct App<'a> {
    exit: bool,
    hypr: Hypr,
    client_table: ClientTable<'a>,
    workspace_table: WorkspaceTable<'a>,
    current_screen: SelectedScreen,
}

impl<'a> App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn new() -> Result<App<'a>> {
        let hypr = Hypr::new().context("Getting information from Hyprland")?;
        let client_table = ClientTable::new(&hypr.clients);
        let workspace_table = WorkspaceTable::new(&hypr.workspaces);
        Ok(App {
            exit: false,
            hypr,
            client_table,
            workspace_table,
            current_screen: SelectedScreen::Clients,
        })
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
            KeyCode::Tab => self.next_border_screen(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_border_screen(&mut self) {
        match self.current_screen {
            SelectedScreen::Clients => self.current_screen = SelectedScreen::Workspaces,
            SelectedScreen::Workspaces => self.current_screen = SelectedScreen::Clients,
        };
    }

    fn border_title(&self) -> Vec<Span<'a>> {
        let mut lines: Vec<Span> = vec![" Clients ".into(), "|".into(), " Workspaces ".into()];

        match self.current_screen {
            SelectedScreen::Clients => lines[0] = lines[0].clone().blue(),
            SelectedScreen::Workspaces => lines[2] = lines[2].clone().blue(),
        };

        lines
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Help ".into(),
            "<?>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Block::bordered()
            .title_top(self.border_title())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);

        let area = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        match self.current_screen {
            SelectedScreen::Clients => self.client_table.render(area, buf),
            SelectedScreen::Workspaces => self.workspace_table.render(area, buf),
        };
    }
}
