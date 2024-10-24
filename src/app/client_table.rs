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
}

impl<'a> ClientTable<'_> {
    pub fn new(clients: &Vec<Client>) -> ClientTable<'a> {
        let state = TableState::default().with_selected(Some(0));
        let widths = [
            Constraint::Max(10),
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

        ClientTable { state, table }
    }
}

impl WidgetRef for ClientTable<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.table.clone(), area, buf, &mut self.state.clone());
    }
}
