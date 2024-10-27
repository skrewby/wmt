use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Widget, WidgetRef},
};

use crate::{
    app::{client_table::ClientTable, workspace_table::WorkspaceTable},
    hypr::Hypr,
};

use super::{
    help_screen::HelpScreen, send_workspace_screen::SendWorkspaceScreen, Screen, ScreenEvent,
    ScreenWidget,
};

enum SelectedTable {
    Clients,
    Workspaces,
}

pub struct TableScreen<'a> {
    client_table: ClientTable<'a>,
    workspace_table: WorkspaceTable<'a>,
    current_table: SelectedTable,
}

impl<'a> TableScreen<'_> {
    pub fn new() -> Result<TableScreen<'a>> {
        let hypr = Hypr::new().context("Connecting to Hyprland")?;
        let client_table = ClientTable::new(hypr.clients);
        let workspace_table = WorkspaceTable::new(hypr.workspaces);
        Ok(TableScreen {
            client_table,
            workspace_table,
            current_table: SelectedTable::Clients,
        })
    }

    fn next_border_screen(&mut self) -> Option<ScreenEvent> {
        match self.current_table {
            SelectedTable::Clients => self.current_table = SelectedTable::Workspaces,
            SelectedTable::Workspaces => self.current_table = SelectedTable::Clients,
        };

        None
    }

    fn border_title(&self) -> Vec<Span<'a>> {
        let mut lines: Vec<Span> = vec![" Clients ".into(), "|".into(), " Workspaces ".into()];

        match self.current_table {
            SelectedTable::Clients => lines[0] = lines[0].clone().blue(),
            SelectedTable::Workspaces => lines[2] = lines[2].clone().blue(),
        };

        lines
    }

    fn table_move_down(&mut self) -> Option<ScreenEvent> {
        match self.current_table {
            SelectedTable::Clients => self.client_table.move_down(),
            SelectedTable::Workspaces => self.workspace_table.move_down(),
        };

        None
    }

    fn table_move_up(&mut self) -> Option<ScreenEvent> {
        match self.current_table {
            SelectedTable::Clients => self.client_table.move_up(),
            SelectedTable::Workspaces => self.workspace_table.move_up(),
        };

        None
    }

    fn switch_to_selected_workspace(&mut self) -> Option<ScreenEvent> {
        let (id_option, client_address) = match self.current_table {
            SelectedTable::Clients => self.client_table.selected_workspace(),
            SelectedTable::Workspaces => (self.workspace_table.selected_workspace(), None),
        };
        if let Some(id) = id_option {
            if let Ok(_) = crate::hypr::switch_to_workspace(id, client_address) {
                return Some(ScreenEvent::Close);
            }
        }

        None
    }

    fn switch_to_workspace(&mut self, id: u32) -> Option<ScreenEvent> {
        if let Ok(_) = crate::hypr::switch_to_workspace(id, None) {
            return Some(ScreenEvent::Close);
        }

        None
    }

    fn send_selected_client_to_workspace(&mut self) -> Option<ScreenEvent> {
        if let Some(client_address) = self.client_table.selected_client() {
            return Some(ScreenEvent::AddScreen(Box::new(SendWorkspaceScreen::new(
                client_address,
            ))));
        };

        None
    }
}

impl WidgetRef for TableScreen<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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

impl ScreenWidget for TableScreen<'_> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenEvent> {
        match key_event.code {
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

            KeyCode::Char('?') => Some(ScreenEvent::AddScreen(Box::new(HelpScreen::new()))),
            KeyCode::Char('s') => self.send_selected_client_to_workspace(),

            KeyCode::Tab => self.next_border_screen(),
            _ => None,
        }
    }

    fn refresh(&mut self) {
        if let Ok(hypr) = Hypr::new().context("Connecting to Hyprland") {
            self.client_table = ClientTable::new(hypr.clients);
            self.workspace_table = WorkspaceTable::new(hypr.workspaces);
        }
    }
}

impl Screen for TableScreen<'_> {}
