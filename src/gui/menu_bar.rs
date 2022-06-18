use crate::gui::main_window::ApplicationState;
use druid::{Data, Env, Menu, MenuItem, WindowId};
use druid::menu::MenuEntry;

pub fn make<T: Data>(_window: Option<WindowId>, _data: &ApplicationState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    base.entry(application()).entry(file())
}

fn file<T: Data>() -> Menu<T> {
    Menu::new("File")
        .entry(druid::platform_menus::win::file::open())
        .entry(druid::platform_menus::win::file::save_as())
}

fn application<T: Data>() -> Menu<T> {
    Menu::new("Application")
        .entry(druid::platform_menus::win::file::exit())
        .entry(MenuItem::new("About").command(druid::commands::SHOW_ABOUT))
}
