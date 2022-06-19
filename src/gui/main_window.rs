use druid::widget::{Flex, Label};
use druid::{AppDelegate, Command, Data, Lens, DelegateCtx, Env, Handled, Target, Widget, WidgetExt, WindowDesc, WindowId, WindowHandle};
use crate::gui::commands::blob_commands;

use crate::gui::image_area::ImageArea;
use crate::gui::sidebar;

/// todo: Consider to extract this somewhere at the top
#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub image_path: String,
}

pub struct Delegate
{
    pub windows: Vec<WindowId>
}

impl AppDelegate<ApplicationState> for Delegate
{
    fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut ApplicationState, _env: &Env) -> Handled {

        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE)
        {
            let image_path = file_info.path().to_str().unwrap().to_string();

            if file_info.path.exists()
            {
                println!("Path to image {}", image_path);
                data.image_path = image_path;
            } else {
                println!("Image path doesn't exist");
            }

            return Handled::Yes;
        }

        if let Some(_) = cmd.get(druid::commands::SHOW_ABOUT)
        {
            // todo: Allow showing about window only once.
            // Attention: WindowId is private and cannot be assigned as a "about_window_id" in ApplicationState
            ctx.new_window(show_about());

            return Handled::Yes;
        }

        if let Some(_) = cmd.get(blob_commands::SHOW_CROP)
        {
            ctx.new_window(show_crop());
        }

        if let Some(_) = cmd.get(blob_commands::SHOW_ROTATE)
        {
            ctx.new_window(show_rotate());
        }

        Handled::No
    }

    fn window_added(&mut self, id: WindowId, _handle: WindowHandle, _data: &mut ApplicationState, _env: &Env, _ctx: &mut DelegateCtx) {
        println!("Window added, id: {:?}", id);
        self.windows.push(id);
    }

    fn window_removed(&mut self, id: WindowId, _data: &mut ApplicationState, _env: &Env, _ctx: &mut DelegateCtx) {
        println!("Window removed, id: {:?}", id);
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}

pub fn build() -> impl Widget<ApplicationState> {
    Flex::row()
        .with_child(sidebar::make())
        .with_spacer(20.0)
        .with_flex_child(ImageArea::new().center(), 1.0)
}

// todo: Move show into separate file
fn show_about() -> WindowDesc<ApplicationState> {
    let label = Label::new("This is an application for image processing thingies.")
        .with_text_size(20.0);

    let about_content = Flex::column().with_child(label);

    WindowDesc::new(about_content).title("About").window_size((250.0, 250.0))
}

// todo: Move show into separate file
fn show_crop() -> WindowDesc<ApplicationState> {
    let label = Label::new("Cropping thingies")
        .with_text_size(20.0);

    let about_content = Flex::column().with_child(label);

    WindowDesc::new(about_content).title("Crop").window_size((250.0, 250.0))
}

// todo: Move show into separate file
fn show_rotate() -> WindowDesc<ApplicationState> {
    let label = Label::new("Rotate thingies")
        .with_text_size(20.0);

    let about_content = Flex::column().with_child(label);

    WindowDesc::new(about_content).title("Rotate").window_size((250.0, 250.0))
}