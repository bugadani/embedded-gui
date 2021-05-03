use core::ops::RangeInclusive;

use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent, ScrollEvent},
    },
    state::{State, StateGroup, WidgetState},
    state_group,
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

    fn handles_scroll(event: ScrollEvent) -> bool;
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

    fn handles_scroll(_event: ScrollEvent) -> bool {
        true
    }
}

impl SliderDirection for Vertical {
    fn xy_to_main_cross<V>(x: V, y: V) -> (V, V) {
        (y, x)
    }
    fn main_cross_to_xy<V>(main: V, cross: V) -> (V, V) {
        (cross, main)
    }
    fn handles_scroll(event: ScrollEvent) -> bool {
        matches!(event, ScrollEvent::VerticalScroll(_))
    }
}

pub trait SliderProperties {
    type Direction: SliderDirection;

    /// Cross axis size of the draggable area.
    const THICKNESS: u32;

    /// Size of the range of values represented by the draggable slider.
    fn length(&self) -> u32;
}

pub struct SliderFields<SP> {
    pub parent_index: usize,
    pub on_value_changed: fn(&mut i32),
    pub value: i32,
    pub limits: RangeInclusive<i32>,
    pub bounds: BoundingBox,
    pub properties: SP,
    pub state: WidgetState,
}

impl<SP> SliderFields<SP>
where
    SP: SliderProperties,
{
    fn value_to_pos(x: i32, x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
        ((y1 - y0) * (x - x0)) / (x1 - x0) + y0
    }

    pub fn set_value(&mut self, value: i32) {
        // TODO: clip instead?
        if self.limits.contains(&value) {
            self.value = value;
        }
    }

    pub fn slider_bounds(&self) -> BoundingBox {
        let total_size = SP::Direction::main_axis_size(self.bounds);
        let slider_length = self.properties.length();
        let space = total_size - slider_length;

        let pos = Self::value_to_pos(
            self.value,
            *self.limits.start(),
            *self.limits.end(),
            0,
            space as i32,
        );
        let (x, y) = SP::Direction::main_cross_to_xy(pos, 0);
        let (width, height) = SP::Direction::main_cross_to_xy(slider_length, SP::THICKNESS);
        BoundingBox {
            position: self.bounds.position + Position { x, y },
            size: MeasuredSize { width, height },
        }
    }
}

state_group! {
    [SliderStateGroup: 0x0000_0003] = {
        Idle = 0,
        Hovered = 0x0000_0001,
        Dragged = 0x0000_0002,
    }

    [SliderInactiveStateGroup: 0x0000_0004] = {
        Active = 0,
        Inactive = 0x0000_0004,
    }
}

impl Slider<(), ()> {
    pub const STATE_IDLE: Idle = Idle;
    pub const STATE_HOVERED: Hovered = Hovered;
    pub const STATE_DRAGGED: Dragged = Dragged;
    pub const STATE_INACTIVE: Inactive = Inactive;
    pub const STATE_ACTIVE: Active = Active;
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
                state: WidgetState::default(),
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

impl<SP, D> Slider<SP, D>
where
    SP: SliderProperties,
    D: WidgetData,
{
    pub fn on_data_changed(mut self, callback: fn(&mut SliderFields<SP>, &D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.data_holder.on_data_changed = callback;
        self
    }
}

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
        let (main_spec, cross_spec) =
            SP::Direction::xy_to_main_cross(measure_spec.width, measure_spec.height);

        let cross_size = cross_spec.apply_to_measured(SP::THICKNESS);
        let main_size = main_spec.largest().unwrap_or(0);

        let (width, height) = SP::Direction::main_cross_to_xy(main_size, cross_size);

        self.set_measured_size(MeasuredSize { width, height });
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        if self.fields.state.has_state(Slider::STATE_INACTIVE) {
            return None;
        }

        match event {
            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                if self.bounding_box().contains(position) {
                    self.fields.state.set_state(Slider::STATE_HOVERED);
                    // We deliberately don't handle hover events. In case the slider is partially
                    // displayed, handling hover would route clicks that fall on the hidden parts.
                } else {
                    // Make sure we reset our state if we don't handle the pointer event.
                    // It's possible we were the target for the last one.
                    self.fields.state.set_state(Slider::STATE_IDLE);
                }
                None
            }

            InputEvent::ScrollEvent(scroll) => {
                if self.fields.state.has_state(Slider::STATE_HOVERED)
                    && SP::Direction::handles_scroll(scroll)
                {
                    Some(0)
                } else {
                    None
                }
            }

            _ => None,
        }

        // We want to handle drags, scrolls with wheel, maybe even arrow key presses.
        // Scroll and arrow handling should depend on direction.
        // Scrollwheel/arrows should change the value, dragging should change position directly.
    }

    fn handle_input(&mut self, _ctxt: InputContext, event: InputEvent) -> bool {
        if self.fields.state.has_state(Slider::STATE_INACTIVE) {
            return false;
        }

        match event {
            InputEvent::Cancel => {}
            InputEvent::KeyEvent(_) => {}
            InputEvent::PointerEvent(_, _) => {}
            InputEvent::ScrollEvent(scroll) => {
                let delta = match scroll {
                    ScrollEvent::HorizontalScroll(delta) => delta,
                    ScrollEvent::VerticalScroll(delta) => -delta,
                };

                // TODO: make this configurable
                const INCREMENT: i32 = 1;
                self.fields.set_value(if delta < 0 {
                    self.fields.value - INCREMENT
                } else {
                    self.fields.value + INCREMENT
                });

                return true;
            }
        }

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
