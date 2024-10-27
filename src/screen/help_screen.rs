use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Margin, Offset, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Row, Table, Widget, WidgetRef},
};

use super::{Screen, ScreenEvent, ScreenWidget};

pub struct HelpScreen {}

impl HelpScreen {
    pub fn new() -> HelpScreen {
        HelpScreen {}
    }

    fn render_keybinds(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Keybinds".blue())
            .alignment(Alignment::Center)
            .render(area, buf);

        let area = area.offset(Offset { x: 0, y: 1 });
        self.render_table_screen_keybinds(area, buf);
        let area = area.offset(Offset { x: 0, y: 9 });
        self.render_table_send_workspace(area, buf);
    }

    fn render_table_screen_keybinds(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Main Screen".bold())
            .alignment(Alignment::Left)
            .render(area, buf);
        let area = area.offset(Offset { x: 0, y: 1 });

        let widths = [Constraint::Max(20), Constraint::Max(10)];
        let rows = [
            Row::new(vec!["Down", "j"]),
            Row::new(vec!["Down", "Arrow Down"]),
            Row::new(vec!["Up", "k"]),
            Row::new(vec!["Up", "Arrow Up"]),
            Row::new(vec!["Focus client", "Enter"]),
            Row::new(vec!["Switch workspace", "0-9"]),
            Row::new(vec!["Send to workspace", "s"]),
        ];
        Table::new(rows, widths).render(area, buf);
    }

    fn render_table_send_workspace(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Send to Workspace Mode".bold())
            .alignment(Alignment::Left)
            .render(area, buf);
        let area = area.offset(Offset { x: 0, y: 1 });

        let widths = [Constraint::Max(20), Constraint::Max(10)];
        let rows = [
            Row::new(vec!["Choose workspace", "Enter"]),
            Row::new(vec!["Choose workspace", "0-9"]),
        ];
        Table::new(rows, widths).render(area, buf);
    }
}

impl WidgetRef for HelpScreen {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Back ".into(),
            "<?>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Block::bordered()
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);

        let area = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        self.render_keybinds(area, buf);
    }
}

impl ScreenWidget for HelpScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenEvent> {
        match key_event.code {
            KeyCode::Char('?') => Some(ScreenEvent::PopScreen),
            _ => None,
        }
    }

    fn refresh(&mut self) {}
}
impl Screen for HelpScreen {}
