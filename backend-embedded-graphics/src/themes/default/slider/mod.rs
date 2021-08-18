use core::{marker::PhantomData, ops::RangeInclusive};

use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Point, Primitive},
    primitives::{Line, PrimitiveStyle},
    Drawable,
};
use embedded_gui::{
    data::WidgetData,
    widgets::slider::{
        Slider, SliderDirection, SliderFields, SliderProperties as SliderPropertiesTrait,
    },
    WidgetRenderer,
};

use crate::{themes::default::DefaultTheme, EgCanvas, ToRectangle};

pub mod binary_color;
pub mod rgb;

pub trait SliderVisualState<C>
where
    C: PixelColor,
{
    const BACKGROUND_LINE_COLOR: Option<C>;
    const BACKGROUND_LINE_THICKNESS: u32 = 0;
    const BORDER_COLOR: Option<C>;
    const FILL_COLOR: Option<C>;
    const BORDER_THICKNESS: u32 = 0;

    fn styles() -> (PrimitiveStyle<C>, PrimitiveStyle<C>) {
        let mut style = PrimitiveStyle::default();
        let mut bg_style = PrimitiveStyle::default();

        style.fill_color = Self::FILL_COLOR;
        style.stroke_width = Self::BORDER_THICKNESS;
        style.stroke_color = Self::BORDER_COLOR;

        bg_style.stroke_width = Self::BACKGROUND_LINE_THICKNESS;
        bg_style.stroke_color = Self::BACKGROUND_LINE_COLOR;

        (style, bg_style)
    }
}

pub trait SliderVisualStyle<C>: Default
where
    C: PixelColor,
{
    type Direction: SliderDirection;

    const THICKNESS: u32;
    const WIDTH: u32;

    type Idle: SliderVisualState<C>;
    type Hovered: SliderVisualState<C>;
    type Dragged: SliderVisualState<C>;
    type Inactive: SliderVisualState<C>;

    fn draw<DT: DrawTarget<Color = C>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<SliderProperties<C, Self>, D>,
    ) -> Result<(), DT::Error> {
        let (slider_style, bg_style) = if slider.state.has_state(Slider::STATE_INACTIVE) {
            Self::Inactive::styles()
        } else if slider.state.has_state(Slider::STATE_DRAGGED) {
            Self::Dragged::styles()
        } else if slider.state.has_state(Slider::STATE_HOVERED) {
            Self::Hovered::styles()
        } else {
            Self::Idle::styles()
        };

        // Background

        let bounds = slider.bounds.to_rectangle();
        let Point { x: x0, y: y0 } = bounds.top_left;
        let Point { x: x1, y: y1 } = bounds.bottom_right().unwrap();

        let y = (y0 + y1) / 2;

        Line::new(Point::new(x0, y), Point::new(x1, y))
            .into_styled(bg_style)
            .draw(&mut canvas.target)?;

        // Foreground
        slider
            .slider_bounds()
            .to_rectangle()
            .into_styled(slider_style)
            .draw(&mut canvas.target)
    }
}

pub struct SliderProperties<C, VS>
where
    C: PixelColor,
    VS: SliderVisualStyle<C>,
{
    visual: VS,
    _marker: PhantomData<C>,
}

impl<C, VS> SliderProperties<C, VS>
where
    C: PixelColor,
    VS: SliderVisualStyle<C>,
{
    pub fn new() -> Self {
        Self {
            visual: VS::default(),
            _marker: PhantomData,
        }
    }
}

impl<C, VS> SliderPropertiesTrait for SliderProperties<C, VS>
where
    C: PixelColor,
    VS: SliderVisualStyle<C>,
{
    type Direction = VS::Direction;
    const THICKNESS: u32 = VS::THICKNESS;

    fn length(&self) -> u32 {
        VS::WIDTH
    }

    fn set_length(&mut self, _length: u32) {
        unimplemented!("Numeric slider has no settable window length")
    }
}

impl<VS, C, DT, D> WidgetRenderer<EgCanvas<DT>> for Slider<SliderProperties<C, VS>, D>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    VS: SliderVisualStyle<C>,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.visual.draw(canvas, &self.fields)
    }
}

pub type StyledSlider<C> = Slider<SliderProperties<C, <C as DefaultTheme>::Slider>, ()>;

pub fn slider<C: DefaultTheme>(range: RangeInclusive<i32>) -> StyledSlider<C> {
    Slider::new(range, SliderProperties::new())
}
