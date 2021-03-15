use crate::{
    widgets::{Widget, WidgetProperties},
    BoundingBox, MeasureSpec,
};

pub struct Button<I: Widget> {
    pub widget_properties: WidgetProperties,
    pub inner: I,
}

impl<I> Button<I>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget_properties: WidgetProperties::default(),
            inner,
        }
    }
}

impl<I: Widget> Widget for Button<I> {
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.inner.bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec)
    }
}
