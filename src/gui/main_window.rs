use druid::widget::Flex;
use druid::{Data, Lens, Widget, WidgetExt};

use crate::gui::image_area::ImageArea;
use crate::gui::sidebar;

/// todo: Consider to extract this somewhere at the top
#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub width: f64,
    pub height: f64,
}

pub fn build() -> impl Widget<ApplicationState> {
    Flex::row()
        .with_child(sidebar::make_sidebar())
        .with_spacer(20.0)
        .with_flex_child(ImageArea::new().center(), 1.0)
}