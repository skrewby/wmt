use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Row, StatefulWidget, Table, TableState, WidgetRef},
};

use crate::hypr::Workspace;

pub struct WorkspaceTable<'a> {
    state: TableState,
    table: Table<'a>,
}

impl<'a> WorkspaceTable<'_> {
    pub fn new(workspaces: &Vec<Workspace>) -> WorkspaceTable<'a> {
        let state = TableState::default().with_selected(Some(0));
        let widths = [
            Constraint::Max(10),
            Constraint::Min(30),
            Constraint::Max(10),
            Constraint::Max(10),
        ];
        let rows: Vec<Row> = workspaces
            .iter()
            .map(|workspace| {
                Row::new(vec![
                    workspace.id.to_string(),
                    workspace.name.clone(),
                    workspace.monitor_id.to_string().clone(),
                    workspace.windows.to_string().clone(),
                ])
            })
            .collect();
        let table = Table::new(rows, widths)
            .header(Row::new(vec!["ID", "Name", "Monitor", "Clients"]).bold())
            .row_highlight_style(Style::new().reversed());

        WorkspaceTable { state, table }
    }
}

impl WidgetRef for WorkspaceTable<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.table.clone(), area, buf, &mut self.state.clone());
    }
}
