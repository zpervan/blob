use druid::widget::prelude::*;
use druid::widget::{Flex, Label};
use druid::{UnitPoint, WidgetExt};

pub fn build() -> impl Widget<u32> {
    let label = Label::new("Welcome to the image processing application!");

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .align_vertical(UnitPoint::CENTER)
}