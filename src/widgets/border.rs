//! Border thickness and color

use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    state::WidgetState,
    widgets::{
        utils::{
            decorator::WidgetDecorator,
            wrapper::{Wrapper, WrapperBindable},
        },
        Widget,
    },
};

pub trait BorderProperties {
    type Color;

    fn set_border_color(&mut self, color: Self::Color);

    fn get_border_width(&self) -> u32;
}

pub struct Border<W, P>
where
    P: BorderProperties,
{
    pub inner: W,
    pub border_properties: P,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<W, P> Border<W, P>
where
    W: Widget,
    P: BorderProperties,
{
    pub fn new(inner: W) -> Border<W, P>
    where
        P: Default,
    {
        Border {
            border_properties: P::default(),
            inner,
            on_state_changed: |_, _| (),
        }
    }
}

impl<W, P> WrapperBindable for Border<W, P>
where
    W: Widget,
    P: BorderProperties,
{
}

impl<W, P> Border<W, P>
where
    P: BorderProperties,
{
    pub fn border_color(mut self, color: P::Color) -> Self {
        self.set_border_color(color);
        self
    }

    pub fn set_border_color(&mut self, color: P::Color) {
        self.border_properties.set_border_color(color);
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<W, P, D> Wrapper<Border<W, P>, D>
where
    W: Widget,
    P: BorderProperties,
    D: WidgetData,
{
    pub fn border_color(mut self, color: P::Color) -> Self {
        self.widget.set_border_color(color);
        self
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Border<W, P>, WidgetState)) -> Self {
        // TODO this should be pulled up
        self.widget.on_state_changed = callback;
        self
    }
}

impl<W, P> WidgetDecorator for Border<W, P>
where
    W: Widget,
    P: BorderProperties,
{
    type Widget = W;

    fn widget(&self) -> &Self::Widget {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut Self::Widget {
        &mut self.inner
    }

    fn arrange(&mut self, position: Position) {
        let bw = self.border_properties.get_border_width();

        self.inner.arrange(Position {
            x: position.x + bw as i32,
            y: position.y + bw as i32,
        });
    }

    fn bounding_box(&self) -> BoundingBox {
        let bw = self.border_properties.get_border_width();
        let bounds = self.inner.bounding_box();

        BoundingBox {
            position: Position {
                x: bounds.position.x - bw as i32,
                y: bounds.position.y - bw as i32,
            },
            size: MeasuredSize {
                width: bounds.size.width + 2 * bw,
                height: bounds.size.height + 2 * bw,
            },
        }
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let bw = self.border_properties.get_border_width();

        self.inner.measure(MeasureSpec {
            width: measure_spec.width.shrink(2 * bw),
            height: measure_spec.height.shrink(2 * bw),
        });
    }

    fn fire_on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }
}
