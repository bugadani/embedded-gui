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
    Canvas, WidgetRenderer,
};

#[derive(Default, Clone, Copy)]
pub struct SpacingSpec {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

pub struct Spacing<W> {
    pub inner: W,
    pub spacing: SpacingSpec,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<W> Spacing<W>
where
    W: Widget,
{
    pub fn new(inner: W) -> Spacing<W> {
        Spacing {
            spacing: SpacingSpec::default(),
            inner,
            on_state_changed: |_, _| (),
        }
    }

    pub fn left(mut self, space: u32) -> Self {
        self.spacing.left = space;
        self
    }

    pub fn right(mut self, space: u32) -> Self {
        self.spacing.right = space;
        self
    }

    pub fn top(mut self, space: u32) -> Self {
        self.spacing.top = space;
        self
    }

    pub fn bottom(mut self, space: u32) -> Self {
        self.spacing.bottom = space;
        self
    }

    pub fn all(mut self, space: u32) -> Self {
        self = self.top(space);
        self = self.right(space);
        self = self.bottom(space);
        self = self.left(space);
        self
    }
}

impl<W> WrapperBindable for Spacing<W> where W: Widget {}

impl<W, D> Wrapper<Spacing<W>, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn left(mut self, space: u32) -> Self {
        self.widget = self.widget.left(space);
        self
    }

    pub fn right(mut self, space: u32) -> Self {
        self.widget = self.widget.right(space);
        self
    }

    pub fn top(mut self, space: u32) -> Self {
        self.widget = self.widget.top(space);
        self
    }

    pub fn bottom(mut self, space: u32) -> Self {
        self.widget = self.widget.bottom(space);
        self
    }

    pub fn all(mut self, space: u32) -> Self {
        self.widget = self.widget.all(space);
        self
    }
}

impl<W> WidgetDecorator for Spacing<W>
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

    fn arrange(&mut self, position: Position) {
        let spacing = self.spacing;

        self.inner.arrange(Position {
            x: position.x + spacing.left as i32,
            y: position.y + spacing.top as i32,
        });
    }

    fn bounding_box(&self) -> BoundingBox {
        let spacing = self.spacing;
        let bounds = self.inner.bounding_box();

        BoundingBox {
            position: Position {
                x: bounds.position.x - spacing.left as i32,
                y: bounds.position.y - spacing.top as i32,
            },
            size: MeasuredSize {
                width: bounds.size.width + spacing.left + spacing.right,
                height: bounds.size.height + spacing.top + spacing.bottom,
            },
        }
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let spacing = self.spacing;

        self.inner.measure(MeasureSpec {
            width: measure_spec.width.shrink(spacing.left + spacing.right),
            height: measure_spec.height.shrink(spacing.top + spacing.bottom),
        });
    }

    fn fire_on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }
}

impl<C, W> WidgetRenderer<C> for Spacing<W>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}
