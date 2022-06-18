use druid::widget::Flex;
use druid::{AppDelegate, Command, Data, Lens, DelegateCtx, Env, Handled, Target, Widget, WidgetExt, FileInfo};

use crate::gui::image_area::ImageArea;
use crate::gui::sidebar;

/// todo: Consider to extract this somewhere at the top
#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub image_path: String
}

pub struct Delegate;

impl AppDelegate<ApplicationState> for Delegate
{
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut ApplicationState, _env: &Env) -> Handled {
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
        } else {
            Handled::No
        }
    }
}

pub fn build() -> impl Widget<ApplicationState> {
    Flex::row()
        .with_child(sidebar::make())
        .with_spacer(20.0)
        .with_flex_child(ImageArea::new().center(), 1.0)
}