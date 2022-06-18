use crate::gui::main_window::ApplicationState;
use druid::{Data, Env, Menu, WindowId};
use std::borrow::BorrowMut;

pub fn make<T: Data>(_window: Option<WindowId>, _data: &ApplicationState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    base.entry(application()).entry(file())
}

fn file<T: Data>() -> Menu<T> {
    Menu::new("File")
        .entry(druid::platform_menus::win::file::open())
        .entry(druid::platform_menus::win::file::save_as())
}

/// todo: Add about dialog
fn application<T: Data>() -> Menu<T> {
    Menu::new("Application").entry(druid::platform_menus::win::file::exit())
}
