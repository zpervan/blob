use druid::widget::{Image, SizedBox};
use druid::widget::{prelude::*, FillStrat};
use druid::{Color, Data, ImageBuf, WidgetExt};
use gui::main_window::ApplicationState;

use crate::gui;

pub struct ImageArea {
    inner: Box<dyn Widget<ApplicationState>>,
}

impl ImageArea {
    pub fn new() -> ImageArea {
        ImageArea {
            inner: SizedBox::empty().boxed(),
        }
    }

    pub fn render(&mut self, data: &ApplicationState) {
        self.inner = render_image(data);
    }
}

fn render_image(_data: &ApplicationState) -> Box<dyn Widget<ApplicationState>> {
    /// todo: Dynamically load image into image view area
    let image_data = ImageBuf::from_data(include_bytes!("./../../assets/s_n_p_1.png")).unwrap();

    let mut img = Image::new(image_data).fill_mode(FillStrat::Fill);
    let mut image_area = SizedBox::new(img);

    image_area.border(Color::grey(0.6), 2.0).center().boxed()
}

impl Widget<ApplicationState> for ImageArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut ApplicationState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &ApplicationState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.render(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &ApplicationState, data: &ApplicationState, env: &Env) {
        if !old_data.same(data) {
            self.render(data);
            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &ApplicationState, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ApplicationState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}
