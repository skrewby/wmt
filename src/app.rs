mod client_table;
mod workspace_table;

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

enum SelectedTable {
    Clients,
    Workspaces,
}

pub struct App<'a> {
    exit: bool,
    client_table: ClientTable<'a>,
    workspace_table: WorkspaceTable<'a>,
    current_table: SelectedTable,
}

impl<'a> App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn new() -> Result<App<'a>> {
        let hypr = Hypr::new().context("Connecting to Hyprland")?;
        let client_table = ClientTable::new(hypr.clients);
        let workspace_table = WorkspaceTable::new(hypr.workspaces);
        Ok(App {
            exit: false,
            client_table,
            workspace_table,
            current_table: SelectedTable::Clients,
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

            KeyCode::Down => self.table_move_down(),
            KeyCode::Char('j') => self.table_move_down(),

            KeyCode::Up => self.table_move_up(),
            KeyCode::Char('k') => self.table_move_up(),

            KeyCode::Enter => self.switch_to_selected_workspace(),
            KeyCode::Char('0') => self.switch_to_workspace(0),
            KeyCode::Char('1') => self.switch_to_workspace(1),
            KeyCode::Char('2') => self.switch_to_workspace(2),
            KeyCode::Char('3') => self.switch_to_workspace(3),
            KeyCode::Char('4') => self.switch_to_workspace(4),
            KeyCode::Char('5') => self.switch_to_workspace(5),
            KeyCode::Char('6') => self.switch_to_workspace(6),
            KeyCode::Char('7') => self.switch_to_workspace(7),
            KeyCode::Char('8') => self.switch_to_workspace(8),
            KeyCode::Char('9') => self.switch_to_workspace(9),

            KeyCode::Tab => self.next_border_screen(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_border_screen(&mut self) {
        match self.current_table {
            SelectedTable::Clients => self.current_table = SelectedTable::Workspaces,
            SelectedTable::Workspaces => self.current_table = SelectedTable::Clients,
        };
    }

    fn border_title(&self) -> Vec<Span<'a>> {
        let mut lines: Vec<Span> = vec![" Clients ".into(), "|".into(), " Workspaces ".into()];

        match self.current_table {
            SelectedTable::Clients => lines[0] = lines[0].clone().blue(),
            SelectedTable::Workspaces => lines[2] = lines[2].clone().blue(),
        };

        lines
    }

    fn table_move_down(&mut self) {
        match self.current_table {
            SelectedTable::Clients => self.client_table.move_down(),
            SelectedTable::Workspaces => self.workspace_table.move_down(),
        }
    }

    fn table_move_up(&mut self) {
        match self.current_table {
            SelectedTable::Clients => self.client_table.move_up(),
            SelectedTable::Workspaces => self.workspace_table.move_up(),
        }
    }

    fn switch_to_selected_workspace(&mut self) {
        let id_option = match self.current_table {
            SelectedTable::Clients => self.client_table.selected_workspace(),
            SelectedTable::Workspaces => self.workspace_table.selected_workspace(),
        };
        if let Some(id) = id_option {
            if let Ok(_) = crate::hypr::switch_to_workspace(id) {
                self.exit = true;
            }
        }
    }

    fn switch_to_workspace(&mut self, id: u32) {
        if let Ok(_) = crate::hypr::switch_to_workspace(id) {
            self.exit = true;
        }
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

        match self.current_table {
            SelectedTable::Clients => self.client_table.render(area, buf),
            SelectedTable::Workspaces => self.workspace_table.render(area, buf),
        };
    }
}
