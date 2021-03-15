use crate::{BoundingBox, MeasureSpec, MeasuredSize, Position, Size};

pub mod button;
pub mod label;

pub trait Widget {
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
