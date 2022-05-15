use druid::widget::Flex;
use druid::{Data, ImageBuf, Lens, Widget, WidgetExt};

use crate::gui::image_widget::ImageArea;

/// todo: Consider to extract this somewhere at the top
#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub width: f64,
    pub height: f64,
}

pub fn build() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_flex_child(ImageArea::new().center(), 1.0)
        .padding(10.0)
}