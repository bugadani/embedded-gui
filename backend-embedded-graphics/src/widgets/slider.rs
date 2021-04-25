use core::{marker::PhantomData, ops::RangeInclusive};

use embedded_graphics::draw_target::DrawTarget;
use embedded_gui::{
    data::WidgetData,
    geometry::{BoundingBox, MeasuredSize},
    widgets::slider::{
        Horizontal, Slider, SliderDirection, SliderProperties as SliderPropertiesTrait, Vertical,
    },
    WidgetRenderer,
};

use crate::EgCanvas;

pub trait SliderStyle {
    /// Cross axis size of the draggable area.
    const THICKNESS: u32;

    /// Smallest possible main axis size of the draggable area.
    const MIN_LENGTH: u32;

    // TODO: these need:
    // - the whole bounding box of the widget
    // - the position and length of the slider
    fn draw_horizontal<DT: DrawTarget>(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error>;
    fn draw_vertical<DT: DrawTarget>(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error>;
}

pub struct SliderProperties<SD, S> {
    _marker: PhantomData<SD>,
    style: S,
}

impl<SD, S> SliderPropertiesTrait for SliderProperties<SD, S>
where
    SD: SliderDirection,
    S: SliderStyle,
{
    type Direction = SD;

    fn slider_size(&self) -> MeasuredSize {
        let (width, height) = SD::main_cross_to_xy(S::MIN_LENGTH, S::THICKNESS);
        MeasuredSize { width, height }
    }
}

impl<S, D, DT> WidgetRenderer<EgCanvas<DT>> for Slider<SliderProperties<Horizontal, S>, D>
where
    D: WidgetData,
    DT: DrawTarget,
    S: SliderStyle,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.style.draw_horizontal(canvas)
    }
}

impl<S, D, DT> WidgetRenderer<EgCanvas<DT>> for Slider<SliderProperties<Vertical, S>, D>
where
    D: WidgetData,
    DT: DrawTarget,
    S: SliderStyle,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.style.draw_vertical(canvas)
    }
}
