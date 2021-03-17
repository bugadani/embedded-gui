use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    input::InputEvent,
    widgets::{Widget, WidgetDataHolder, WidgetProperties, WidgetWrapper},
    BoundingBox, InputCtxt, MeasureSpec, MeasuredSize, Position,
};

#[derive(Default, Clone, Copy)]
pub struct SpacingSpec {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

pub struct Spacing<W, D>
where
    D: WidgetData,
{
    pub inner: W,
    pub spacing: SpacingSpec,
    pub _marker: PhantomData<D>,
}

impl<W> Spacing<W, NoData>
where
    W: Widget,
{
    pub fn new(inner: W) -> WidgetWrapper<Spacing<W, NoData>, NoData> {
        WidgetWrapper {
            widget: Spacing {
                spacing: SpacingSpec::default(),
                inner,
                _marker: PhantomData,
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W, D> Spacing<W, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn set_left(&mut self, space: u32) {
        self.spacing.left = space;
    }
    pub fn set_right(&mut self, space: u32) {
        self.spacing.right = space;
    }
    pub fn set_top(&mut self, space: u32) {
        self.spacing.top = space;
    }
    pub fn set_bottom(&mut self, space: u32) {
        self.spacing.bottom = space;
    }
}

impl<W> Spacing<W, NoData>
where
    W: Widget,
{
    pub fn bind<D>(self) -> Spacing<W, D>
    where
        D: WidgetData,
    {
        Spacing {
            inner: self.inner,
            spacing: self.spacing,
            _marker: PhantomData,
        }
    }
}

impl<W> WidgetWrapper<Spacing<W, NoData>, NoData>
where
    W: Widget,
{
    pub fn bind<D>(self, data: D) -> WidgetWrapper<Spacing<W, D>, D>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            widget: self.widget.bind::<D>(),
            data_holder: self.data_holder.bind(data),
        }
    }
}

impl<W, D> WidgetWrapper<Spacing<W, D>, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn left(mut self, space: u32) -> Self {
        self.widget.set_left(space);
        self
    }

    pub fn right(mut self, space: u32) -> Self {
        self.widget.set_right(space);
        self
    }

    pub fn top(mut self, space: u32) -> Self {
        self.widget.set_top(space);
        self
    }

    pub fn bottom(mut self, space: u32) -> Self {
        self.widget.set_bottom(space);
        self
    }
}

impl<W, D> Widget for WidgetWrapper<Spacing<W, D>, D>
where
    W: Widget,
    D: WidgetData,
{
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        self.widget.inner.widget_properties()
    }

    fn arrange(&mut self, position: Position) {
        let spacing = self.widget.spacing;

        self.widget.inner.arrange(Position {
            x: position.x + spacing.left as i32,
            y: position.y + spacing.top as i32,
        });
    }

    fn bounding_box(&self) -> BoundingBox {
        let spacing = self.widget.spacing;
        let bounds = self.widget.inner.bounding_box();

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

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        // todo modify measure_spec
        self.widget.inner.measure(measure_spec);
    }

    fn children(&self) -> usize {
        1 + self.widget.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.widget.inner
        } else {
            self.widget.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.widget.inner
        } else {
            self.widget.inner.get_mut_child(idx - 1)
        }
    }

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        self.widget.inner.handle_input(ctxt, event)
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
