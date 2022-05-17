use druid::widget::{Button, Flex, Label};
use druid::{UnitPoint, Widget, WidgetExt};

use crate::ApplicationState;

fn make_filters_section() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_child(Label::new("FILTERS"))
        .with_child(Button::new("Median").padding(5.0))
        .with_spacer(20.0)
}

fn make_generators_section() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_child(Label::new("GENERATORS"))
        .with_child(
            Flex::row()
                .with_child(Button::new("Gradient").padding(5.0))
                .with_child(Button::new("Fractal").padding(5.0))
        )
        .with_spacer(20.0)
}

fn make_image_section() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_child(Label::new("IMAGE"))
        .with_child(
            Flex::row()
                .with_child(Button::new("Crop").padding(5.0))
                .with_child(Button::new("Rotate").padding(5.0))
        )
        .with_spacer(20.0)
}

fn make_pixel_section() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_child(Label::new("PIXEL MANIPULATION"))
        .with_child(
            Flex::row()
                .with_child(Button::new("Blur").padding(5.0))
                .with_child(Button::new("Brighten").padding(5.0))
                .with_child(Button::new("Grayscale").padding(5.0))
                .with_child(Button::new("Invert").padding(5.0)),
        )
}

pub fn make_sidebar() -> impl Widget<ApplicationState> {
    Flex::column()
        .with_child(make_filters_section())
        .with_child(make_generators_section())
        .with_child(make_image_section())
        .with_child(make_pixel_section())
        .align_horizontal(UnitPoint::LEFT)
}
