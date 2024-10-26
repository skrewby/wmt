use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;

pub mod table_screen;

pub enum ScreenEvent {
    Close,
}

pub trait ScreenWidget {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenEvent>;
}

pub trait Screen: ScreenWidget + WidgetRef {}
