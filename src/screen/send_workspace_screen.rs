use anyhow::Context;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget, WidgetRef},
};

use crate::{app::workspace_table::WorkspaceTable, hypr::Hypr};

use super::{help_screen::HelpScreen, Screen, ScreenEvent, ScreenWidget};

pub struct SendWorkspaceScreen<'a> {
    workspace_table: WorkspaceTable<'a>,
    client_address: String,
}

impl<'a> SendWorkspaceScreen<'_> {
    pub fn new(client_address: String) -> SendWorkspaceScreen<'a> {
        let hypr = Hypr::new().context("Connecting to Hyprland").unwrap();
        let workspace_table = WorkspaceTable::new(hypr.workspaces);
        SendWorkspaceScreen {
            workspace_table,
            client_address,
        }
    }

    fn table_move_down(&mut self) -> Option<ScreenEvent> {
        self.workspace_table.move_down();

        None
    }

    fn table_move_up(&mut self) -> Option<ScreenEvent> {
        self.workspace_table.move_up();

        None
    }

    fn send_to_selected_workspace(&mut self) -> Option<ScreenEvent> {
        let id_option = self.workspace_table.selected_workspace();
        if let Some(id) = id_option {
            if let Ok(_) = crate::hypr::send_to_workspace(id, self.client_address.clone()) {
                return Some(ScreenEvent::Close);
            }
        }

        None
    }

    fn send_to_workspace(&mut self, id: u32) -> Option<ScreenEvent> {
        if let Ok(_) = crate::hypr::send_to_workspace(id, self.client_address.clone()) {
            return Some(ScreenEvent::Close);
        }

        None
    }
}

impl WidgetRef for SendWorkspaceScreen<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Back ".into(),
            "<ESC>".blue().bold(),
            " Help ".into(),
            "<?>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Block::bordered()
            .title_top(" Send to Workspace ")
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);

        let area = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        self.workspace_table.render(area, buf);
    }
}

impl ScreenWidget for SendWorkspaceScreen<'_> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenEvent> {
        match key_event.code {
            KeyCode::Down => self.table_move_down(),
            KeyCode::Char('j') => self.table_move_down(),

            KeyCode::Up => self.table_move_up(),
            KeyCode::Char('k') => self.table_move_up(),

            KeyCode::Esc => Some(ScreenEvent::PopScreen),
            KeyCode::Char('?') => Some(ScreenEvent::AddScreen(Box::new(HelpScreen::new()))),

            KeyCode::Enter => self.send_to_selected_workspace(),
            KeyCode::Char('0') => self.send_to_workspace(0),
            KeyCode::Char('1') => self.send_to_workspace(1),
            KeyCode::Char('2') => self.send_to_workspace(2),
            KeyCode::Char('3') => self.send_to_workspace(3),
            KeyCode::Char('4') => self.send_to_workspace(4),
            KeyCode::Char('5') => self.send_to_workspace(5),
            KeyCode::Char('6') => self.send_to_workspace(6),
            KeyCode::Char('7') => self.send_to_workspace(7),
            KeyCode::Char('8') => self.send_to_workspace(8),
            KeyCode::Char('9') => self.send_to_workspace(9),

            _ => None,
        }
    }
}
impl Screen for SendWorkspaceScreen<'_> {}
