mod algorithms;
mod image_processing;
mod constants;
mod gui;

use druid::{WindowDesc, AppLauncher};
use gui::main_window::ApplicationState;

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(gui::main_window::build())
        .title("Image Processing Application")
        .window_size((constants::DEFAULT_WINDOW_WIDTH, constants::DEFAULT_WINDOW_HEIGHT));

    let application_state = ApplicationState
    {
        width: 200.0,
        height: 100.0,
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(application_state)
        .expect("Failed to launch the application");
}
