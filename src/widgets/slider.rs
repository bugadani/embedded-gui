use core::ops::RangeInclusive;

use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    input::{controller::InputContext, event::InputEvent},
    state::WidgetState,
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
};

pub trait SliderDirection {
    fn main_cross_to_xy<V>(main: V, cross: V) -> (V, V);
    fn xy_to_main_cross<V>(x: V, y: V) -> (V, V);

    fn main_axis_size(bounds: BoundingBox) -> u32 {
        let (main, _) = Self::xy_to_main_cross(bounds.size.width, bounds.size.height);

        main
    }

    fn cross_axis_size(bounds: BoundingBox) -> u32 {
        let (_, cross) = Self::xy_to_main_cross(bounds.size.width, bounds.size.height);

        cross
    }
}

pub struct Horizontal;

pub struct Vertical;

impl SliderDirection for Horizontal {
    fn xy_to_main_cross<V>(x: V, y: V) -> (V, V) {
        (x, y)
    }
    fn main_cross_to_xy<V>(main: V, cross: V) -> (V, V) {
        (main, cross)
    }
}

impl SliderDirection for Vertical {
    fn xy_to_main_cross<V>(x: V, y: V) -> (V, V) {
        (y, x)
    }
    fn main_cross_to_xy<V>(main: V, cross: V) -> (V, V) {
        (cross, main)
    }
}

pub trait SliderProperties {
    type Direction: SliderDirection;

    /// Size of the draggable area.
    fn slider_size(&self) -> MeasuredSize;
}

pub struct SliderFields<SP> {
    pub parent_index: usize,
    pub on_value_changed: fn(&mut i32),
    pub value: i32,
    pub limits: RangeInclusive<i32>,
    pub bounds: BoundingBox,
    pub properties: SP,
}

pub struct Slider<SP, D = ()>
where
    D: WidgetData,
{
    pub fields: SliderFields<SP>,
    data_holder: WidgetDataHolder<SliderFields<SP>, D>,
}

impl<SP> Slider<SP, ()>
where
    SP: SliderProperties,
{
    pub fn new(limits: RangeInclusive<i32>, properties: SP) -> Self {
        Slider {
            fields: SliderFields {
                parent_index: 0,
                on_value_changed: |_| (),
                value: *limits.start(),
                bounds: BoundingBox::default(),
                limits,
                properties,
            },
            data_holder: WidgetDataHolder::default(),
        }
    }

    pub fn bind<D>(self, data: D) -> Slider<SP, D>
    where
        D: WidgetData,
    {
        Slider {
            fields: self.fields,
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

// TODO: This only applies to scrollbars
// pub fn set_length(&mut self, length: u32) -> &mut Self {
//     let MeasuredSize { width, height } = self.slider_bounds.size;
//
//     let (_, cross) = SD::xy_to_main_cross(width, height);
//     let (width, height) = SD::main_cross_to_xy(length, cross);
//
//     self.slider_bounds.size = MeasuredSize { width, height };
//
//     self
// }

impl<SP, D> WidgetStateHolder for Slider<SP, D>
where
    D: WidgetData,
{
    fn on_state_changed(&mut self, _state: WidgetState) {
        // don't react to parent's state change
    }

    fn is_selectable(&self) -> bool {
        true
    }
}

impl<SP, D> Widget for Slider<SP, D>
where
    SP: SliderProperties,
    D: WidgetData,
{
    fn bounding_box(&self) -> BoundingBox {
        self.fields.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.fields.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        // Measure depends on platform specifics
        let cross = SP::Direction::cross_axis_size(self.fields.properties.slider_bounds());

        let (main_spec, cross_spec) =
            SP::Direction::xy_to_main_cross(measure_spec.width, measure_spec.height);

        let cross_size = cross_spec.apply_to_measured(cross);
        let main_size = main_spec.largest().unwrap_or(0);

        let (width, height) = SP::Direction::main_cross_to_xy(main_size, cross_size);

        self.set_measured_size(MeasuredSize { width, height });
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We want to handle drags, scrolls with wheel, maybe even arrow key presses.
        // Scroll and arrow handling should depend on direction.
        None
    }

    fn handle_input(&mut self, _ctxt: InputContext, event: InputEvent) -> bool {
        false
    }
}

impl<SP, D> UpdateHandler for Slider<SP, D>
where
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.fields);
    }
}

impl<SP, D> ParentHolder for Slider<SP, D>
where
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.fields.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.fields.parent_index = index;
    }
}
