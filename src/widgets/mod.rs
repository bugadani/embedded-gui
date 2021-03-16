use crate::{BoundingBox, MeasureSpec, MeasuredSize, Position, Size};

pub mod button;
pub mod label;

pub trait Widget {
    type Data;
    //type InputController: InputController;

    //fn input_event(&mut self, event: <Self::InputController as InputController>::Event) -> bool;
    fn widget_properties(&mut self) -> &mut WidgetProperties;

    fn bounding_box(&self) -> BoundingBox;

    fn bounding_box_mut(&mut self) -> &mut BoundingBox;

    fn width(mut self, width: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().width = width;
        self
    }

    fn height(mut self, height: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().height = height;
        self
    }

    fn measure(&mut self, measure_spec: MeasureSpec);

    fn arrange(&mut self, position: Position) {
        self.bounding_box_mut().position = position;
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        self.bounding_box_mut().size = size;
    }
}

pub struct WidgetProperties {
    pub width: Size,
    pub height: Size,
}

impl Default for WidgetProperties {
    fn default() -> Self {
        Self {
            width: Size::WrapContent,
            height: Size::WrapContent,
        }
    }
}

pub struct WidgetDataHolder<W, D> {
    pub data: D,
    pub on_data_changed: fn(&mut W, &D),
}

impl<W> Default for WidgetDataHolder<W, ()> {
    fn default() -> Self {
        Self {
            data: (),
            on_data_changed: |_, _| (),
        }
    }
}

impl<W> WidgetDataHolder<W, ()> {
    pub fn bind<W2, D>(self, data: D) -> WidgetDataHolder<W2, D> {
        WidgetDataHolder {
            data,
            on_data_changed: |_, _| (),
        }
    }
}

pub trait DataHolder: Widget {
    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self, Self::Data>
    where
        Self: Sized;

    fn on_data_changed(mut self, callback: fn(&mut Self, &Self::Data)) -> Self
    where
        Self: Sized,
    {
        self.data_holder().on_data_changed = callback;
        self
    }
}
