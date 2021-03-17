use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    widgets::{DataHolder, Widget, WidgetDataHolder, WidgetProperties},
    BoundingBox, Canvas, MeasureConstraint, MeasureSpec, MeasuredSize,
};

pub trait LabelProperties<C: Canvas> {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub trait LabelConstructor<C, P> {
    fn new(text: &'static str) -> Label<C, P, NoData>
    where
        C: Canvas,
        P: LabelProperties<C>;
}

pub struct LabelWidget<C, P, D>
where
    C: Canvas,
    P: LabelProperties<C>,
    D: WidgetData,
{
    // FIXME: use heapless::String
    pub text: &'static str,
    pub widget_properties: WidgetProperties,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub _marker: PhantomData<(C, D)>,
}

impl<C, P> LabelWidget<C, P, NoData>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self) -> LabelWidget<C, P, D>
    where
        D: WidgetData,
    {
        LabelWidget {
            widget_properties: self.widget_properties,
            label_properties: self.label_properties,
            bounds: self.bounds,
            text: self.text,
            _marker: PhantomData,
        }
    }
}

// TODO maybe these wrappers can be merged - WidgetHolder struct which is Widget + DataHolder
pub struct Label<C, P, D>
where
    C: Canvas,
    P: LabelProperties<C>,
    D: WidgetData,
{
    pub widget: LabelWidget<C, P, D>,
    pub data_holder: WidgetDataHolder<LabelWidget<C, P, D>, D>,
}

impl<C, P> Label<C, P, NoData>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self, data: D) -> Label<C, P, D>
    where
        D: WidgetData,
    {
        Label {
            widget: self.widget.bind::<D>(),
            data_holder: self.data_holder.bind(data),
        }
    }
}

impl<C, P, D> DataHolder for Label<C, P, D>
where
    C: Canvas,
    P: LabelProperties<C>,
    D: WidgetData,
{
    type Data = D;
    type Widget = LabelWidget<C, P, D>;

    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self::Widget, Self::Data>
    where
        Self: Sized,
    {
        &mut self.data_holder
    }
}

impl<C, P, D> Widget for Label<C, P, D>
where
    C: Canvas,
    P: LabelProperties<C>,
    D: WidgetData,
{
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.widget.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.widget.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self.widget.label_properties.measure_text(self.widget.text);

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

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
