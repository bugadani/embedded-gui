use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    input::InputEvent,
    widgets::{Widget, WidgetDataHolder, WidgetWrapper},
    BoundingBox, InputCtxt, MeasureSpec, MeasuredSize, Position,
};

pub trait BorderProperties {
    fn get_border_width(&self) -> u32;
}

pub struct Border<I, P, D>
where
    P: BorderProperties,
    D: WidgetData,
{
    pub inner: I,
    pub border_properties: P,
    pub _marker: PhantomData<D>,
}

impl<I, P> Border<I, P, NoData>
where
    I: Widget,
    P: BorderProperties + Default,
{
    pub fn new(inner: I) -> WidgetWrapper<Border<I, P, NoData>, NoData> {
        WidgetWrapper {
            widget: Border {
                border_properties: P::default(),
                inner,
                _marker: PhantomData,
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W, P> Border<W, P, NoData>
where
    W: Widget,
    P: BorderProperties,
{
    pub fn bind<D>(self) -> Border<W, P, D>
    where
        D: WidgetData,
    {
        Border {
            inner: self.inner,
            border_properties: self.border_properties,
            _marker: PhantomData,
        }
    }
}

impl<W, P> WidgetWrapper<Border<W, P, NoData>, NoData>
where
    W: Widget,
    P: BorderProperties,
{
    pub fn bind<D>(self, data: D) -> WidgetWrapper<Border<W, P, D>, D>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            widget: self.widget.bind::<D>(),
            data_holder: self.data_holder.bind(data),
        }
    }
}

impl<W, P, D> Widget for WidgetWrapper<Border<W, P, D>, D>
where
    W: Widget,
    P: BorderProperties,
    D: WidgetData,
{
    fn arrange(&mut self, position: Position) {
        let bw = self.widget.border_properties.get_border_width();

        self.widget.inner.arrange(Position {
            x: position.x + bw as i32,
            y: position.y + bw as i32,
        });
    }

    fn bounding_box(&self) -> BoundingBox {
        let bw = self.widget.border_properties.get_border_width();
        let bounds = self.widget.inner.bounding_box();

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

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let bw = self.widget.border_properties.get_border_width();

        self.widget.inner.measure(MeasureSpec {
            width: measure_spec.width.shrink(2 * bw),
            height: measure_spec.height.shrink(2 * bw),
        });
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
