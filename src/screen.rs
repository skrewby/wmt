use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;

pub mod help_screen;
pub mod send_workspace_screen;
pub mod table_screen;

pub enum ScreenEvent {
    Close,
    AddScreen(Box<dyn Screen>),
    PopScreen,
    PopAndRefresh,
}

pub trait ScreenWidget {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ScreenEvent>;
    fn refresh(&mut self);
}

pub trait Screen: ScreenWidget + WidgetRef {}
