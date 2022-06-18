mod algorithms;
mod image_processing;
mod constants;
mod gui;

use druid::{WindowDesc, AppLauncher};
use gui::main_window::{ApplicationState, Delegate};
use gui::menu_bar;

use std::string::String;

fn main() {
    let main_window = WindowDesc::new(gui::main_window::build())
        .menu(menu_bar::make)
        .title("Image Processing Application")
        .window_size((constants::DEFAULT_WINDOW_WIDTH, constants::DEFAULT_WINDOW_HEIGHT));

    let application_state = ApplicationState
    {
        image_path: String::new(),
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(application_state)
        .expect("Failed to launch the application");
}
