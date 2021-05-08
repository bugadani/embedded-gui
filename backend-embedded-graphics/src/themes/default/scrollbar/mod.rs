use core::{marker::PhantomData, ops::RangeInclusive};

use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Primitive},
    primitives::PrimitiveStyle,
    Drawable,
};
use embedded_gui::{
    data::WidgetData,
    geometry::{BoundingBox, MeasuredSize},
    widgets::slider::{
        Horizontal, Slider, SliderDirection, SliderFields, SliderProperties, Vertical,
    },
    WidgetRenderer,
};

use crate::{themes::default::DefaultTheme, EgCanvas, ToRectangle};

pub mod binary_color;
pub mod rgb;

pub trait ScrollbarVisualState<C>
where
    C: PixelColor,
{
    const BACKGROUND_FILL_COLOR: Option<C>;
    const BACKGROUND_BORDER_COLOR: Option<C>;
    const BACKGROUND_BORDER_THICKNESS: u32 = 0;
    const BORDER_COLOR: Option<C>;
    const FILL_COLOR: Option<C>;
    const BORDER_THICKNESS: u32 = 0;

    fn styles() -> (PrimitiveStyle<C>, PrimitiveStyle<C>) {
        let mut style = PrimitiveStyle::default();
        let mut bg_style = PrimitiveStyle::default();

        style.fill_color = Self::FILL_COLOR;
        style.stroke_width = Self::BORDER_THICKNESS;
        style.stroke_color = Self::BORDER_COLOR;

        bg_style.fill_color = Self::BACKGROUND_FILL_COLOR;
        bg_style.stroke_width = Self::BACKGROUND_BORDER_THICKNESS;
        bg_style.stroke_color = Self::BACKGROUND_BORDER_COLOR;

        (style, bg_style)
    }
}

pub trait ScrollbarVisualStyle<C>: Default
where
    C: PixelColor,
{
    type Direction: SliderDirection;

    const THICKNESS: u32;

    type Idle: ScrollbarVisualState<C>;
    type Hovered: ScrollbarVisualState<C>;
    type Dragged: ScrollbarVisualState<C>;
    type Inactive: ScrollbarVisualState<C>;

    fn draw<DT: DrawTarget<Color = C>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<C, Self>, D>,
    ) -> Result<(), DT::Error>;

    fn draw_horizontal<DT: DrawTarget<Color = C>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<C, Self>, D>,
    ) -> Result<(), DT::Error> {
        todo!()
    }

    fn draw_vertical<DT: DrawTarget<Color = C>, D>(
        &self,
        canvas: &mut crate::EgCanvas<DT>,
        slider: &SliderFields<ScrollbarProperties<C, Self>, D>,
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
        slider
            .bounds
            .to_rectangle()
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

pub struct ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    visual: VS,
    window_length: u32,
    _marker: PhantomData<C>,
}

impl<C, VS> ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    pub fn new() -> Self {
        Self {
            visual: VS::default(),
            window_length: 0,
            _marker: PhantomData,
        }
    }

    pub fn set_length(&mut self, length: u32) {
        self.window_length = length;
    }
}

impl<C, VS> SliderProperties for ScrollbarProperties<C, VS>
where
    C: PixelColor,
    VS: ScrollbarVisualStyle<C>,
{
    type Direction = VS::Direction;
    const THICKNESS: u32 = VS::THICKNESS;

    fn length(&self) -> u32 {
        self.window_length
    }
}

impl<VS, C, DT, D> WidgetRenderer<EgCanvas<DT>> for Slider<ScrollbarProperties<C, VS>, D>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    VS: ScrollbarVisualStyle<C>,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.visual.draw(canvas, &self.fields)
    }
}

pub type StyledVerticalScrollbar<C> =
    Slider<ScrollbarProperties<C, <C as DefaultTheme>::VerticalScrollbar>, ()>;

pub fn vertical_scrollbar<C: DefaultTheme>() -> StyledVerticalScrollbar<C> {
    Slider::new(0..=30, ScrollbarProperties::new())
}
