use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Row, StatefulWidget, Table, TableState, WidgetRef},
};

use crate::hypr::Client;

pub struct ClientTable<'a> {
    state: TableState,
    table: Table<'a>,
    clients: Vec<Client>,
    len: usize,
}

impl<'a> ClientTable<'_> {
    pub fn new(clients: Vec<Client>) -> ClientTable<'a> {
        let state = TableState::default().with_selected(Some(0));
        let widths = [
            Constraint::Max(15),
            Constraint::Min(30),
            Constraint::Max(10),
        ];
        let rows: Vec<Row> = clients
            .iter()
            .map(|client| {
                Row::new(vec![
                    client.class.clone(),
                    client.title.clone(),
                    client.workspace.name.clone(),
                ])
            })
            .collect();
        let table = Table::new(rows, widths)
            .header(Row::new(vec!["Class", "Title", "Workspace"]).bold())
            .row_highlight_style(Style::new().reversed());
        let len = clients.len();

        ClientTable {
            state,
            table,
            clients,
            len,
        }
    }

    pub fn move_down(&mut self) {
        if let Some(i) = self.state.selected() {
            if i < self.len - 1 {
                self.state.select_next();
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(i) = self.state.selected() {
            if i > 0 {
                self.state.select_previous();
            }
        }
    }

    pub fn selected_workspace(&self) -> (Option<u32>, Option<String>) {
        if let Some(index) = self.state.selected() {
            if let Some(client) = self.clients.get(index) {
                return (Some(client.workspace.id), Some(client.address.clone()));
            } else {
                return (None, None);
            }
        } else {
            return (None, None);
        }
    }

    pub fn selected_client(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            if let Some(client) = self.clients.get(index) {
                return Some(client.address.clone());
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

impl WidgetRef for ClientTable<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.table.clone(), area, buf, &mut self.state.clone());
    }
}
