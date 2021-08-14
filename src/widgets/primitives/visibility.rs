use crate::{
    data::WidgetData,
    geometry::{BoundingBox, MeasuredSize},
    input::event::InputEvent,
    state::WidgetState,
    widgets::{
        utils::{
            decorator::WidgetDecorator,
            wrapper::{Wrapper, WrapperBindable},
        },
        Widget,
    },
    Canvas, WidgetRenderer,
};

pub struct Visibility<W> {
    pub inner: W,
    pub visibility: bool,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<W> Visibility<W>
where
    W: Widget,
{
    pub fn new(inner: W) -> Visibility<W> {
        Visibility {
            inner,
            visibility: true,
            on_state_changed: |_, _| (),
        }
    }
}

impl<W> WrapperBindable for Visibility<W> where W: Widget {}

impl<W> Visibility<W> {
    pub fn visible(mut self, visibility: bool) -> Self {
        self.set_visible(visibility);
        self
    }

    pub fn set_visible(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<W, D> Wrapper<Visibility<W>, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn visible(mut self, visibility: bool) -> Self {
        self.widget.set_visible(visibility);
        self
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Visibility<W>, WidgetState)) -> Self {
        // TODO this should be pulled up
        self.widget.on_state_changed = callback;
        self
    }
}

impl<W> WidgetDecorator for Visibility<W>
where
    W: Widget,
{
    type Widget = W;

    fn widget(&self) -> &Self::Widget {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut Self::Widget {
        &mut self.inner
    }

    fn bounding_box(&self) -> BoundingBox {
        if self.visibility {
            self.inner.bounding_box()
        } else {
            BoundingBox {
                position: self.inner.bounding_box().position,
                size: MeasuredSize {
                    width: 0,
                    height: 0,
                },
            }
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        if self.visibility {
            // We just relay whatever the child desires
            self.inner.test_input(event).map(|i| i + 1)
        } else {
            None
        }
    }
}

impl<C, W> WidgetRenderer<C> for Visibility<W>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        if self.visibility {
            self.inner.draw(canvas)
        } else {
            Ok(())
        }
    }
}
