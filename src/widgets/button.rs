use crate::{
    widgets::{DataHolder, Widget, WidgetDataHolder, WidgetProperties},
    BoundingBox, MeasureSpec,
};

pub struct Button<I, D> {
    pub widget_properties: WidgetProperties,
    pub inner: I,
    pub data_holder: WidgetDataHolder<Self, D>,
}

impl<I> Button<I, ()>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget_properties: WidgetProperties::default(),
            inner,
            data_holder: Default::default(),
        }
    }

    pub fn bind<D>(self, data: D) -> Button<I, D>
    where
        Self: Sized,
    {
        Button {
            widget_properties: self.widget_properties,
            inner: self.inner,
            data_holder: self.data_holder.bind(data),
        }
    }
}

impl<I, D> DataHolder for Button<I, D>
where
    I: Widget,
{
    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self, Self::Data>
    where
        Self: Sized,
    {
        &mut self.data_holder
    }
}

impl<I, D> Widget for Button<I, D>
where
    I: Widget,
{
    type Data = D;

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
