use core::marker::PhantomData;

use crate::{
    widgets::{Widget, WidgetProperties},
    BoundingBox, Canvas, MeasureConstraint, MeasureSpec, MeasuredSize,
};

pub trait LabelProperties<C: Canvas> {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub trait LabelConstructor<C, P> {
    fn new(text: &'static str) -> Label<C, P>
    where
        C: Canvas,
        P: LabelProperties<C>;
}

pub struct Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    // FIXME: use heapless::String
    pub text: &'static str,
    pub widget_properties: WidgetProperties,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub _marker: PhantomData<C>,
}

impl<C, P> Widget for Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self.label_properties.measure_text(self.text);

        let width = match measure_spec.width {
            MeasureConstraint::AtMost(width) => width.min(size.width),
            MeasureConstraint::Exactly(width) => width,
            MeasureConstraint::Unspecified => size.width,
        };

        let height = match measure_spec.height {
            MeasureConstraint::AtMost(height) => height.min(size.height),
            MeasureConstraint::Exactly(height) => height,
            MeasureConstraint::Unspecified => size.height,
        };

        self.set_measured_size(MeasuredSize { width, height })
    }
}
