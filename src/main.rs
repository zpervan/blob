mod algorithms;
mod image_processing;
mod constants;
mod gui;

use druid::{WindowDesc, AppLauncher};
use gui::main_window;

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(gui::main_window::build())
        .title("Image Processing Application")
        .window_size((constants::DEFAULT_WINDOW_WIDTH, constants::DEFAULT_WINDOW_HEIGHT));

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(0u32)
        .expect("Failed to launch the application");
}
