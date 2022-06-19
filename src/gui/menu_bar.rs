use crate::gui::main_window::ApplicationState;
use crate::gui::commands::blob_commands;
use druid::{Data, Env, Menu, MenuItem, WindowId};

pub fn make<T: Data>(_window: Option<WindowId>, _data: &ApplicationState, _env: &Env) -> Menu<T> {
    let base = Menu::empty();
    base.entry(file()).entry(image()).entry(application())
}

fn file<T: Data>() -> Menu<T> {
    Menu::new("File")
        .entry(druid::platform_menus::win::file::open())
        .entry(druid::platform_menus::win::file::save_as())
}

fn image<T: Data>() -> Menu<T> {
    Menu::new("Image")
        .entry(MenuItem::new("Crop").command(blob_commands::SHOW_CROP))
        .entry(MenuItem::new("Rotate").command(blob_commands::SHOW_ROTATE))
}

fn application<T: Data>() -> Menu<T> {
    Menu::new("Application")
        .entry(druid::platform_menus::win::file::exit())
        .entry(MenuItem::new("About").command(druid::commands::SHOW_ABOUT))
}
