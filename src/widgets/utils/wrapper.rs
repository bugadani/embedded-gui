use crate::{
    data::WidgetData,
    widgets::{utils::decorator::WidgetDecorator, utils::WidgetDataHolder, Widget},
    Canvas, WidgetRenderer,
};

pub struct Wrapper<W, D>
where
    D: WidgetData,
{
    pub widget: W,
    pub data_holder: WidgetDataHolder<W, D>,
}

/// Trait that lets you bind data to a widget.
///
/// Bound data allows manipulating the widget when the data changes.
pub trait WrapperBindable: Widget + Sized {
    fn bind<D>(self, data: D) -> Wrapper<Self, D>
    where
        D: WidgetData,
    {
        Wrapper {
            widget: self,
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<W, D> Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn on_data_changed(mut self, callback: fn(&mut W, &D::Data)) -> Self {
        self.data_holder.on_data_changed = callback;
        self
    }
}

impl<W, D> WidgetDecorator for Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    type Widget = W;

    fn widget(&self) -> &Self::Widget {
        &self.widget
    }

    fn widget_mut(&mut self) -> &mut Self::Widget {
        &mut self.widget
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
        self.widget.update();
    }

    fn reset_changed(&mut self) {
        self.data_holder.reset_changed();
    }
}

impl<W, D, C> WidgetRenderer<C> for Wrapper<W, D>
where
    W: Widget + WidgetRenderer<C>,
    D: WidgetData,
    C: Canvas,
{
    fn draw(&mut self, canvas: &mut C) -> Result<(), C::Error> {
        self.widget.draw(canvas)
    }
}
