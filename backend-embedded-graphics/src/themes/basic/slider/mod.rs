//! Macros and helpers to draw numeric sliders.

use core::{marker::PhantomData, ops::RangeInclusive};

use crate::{themes::basic::BasicTheme, EgCanvas, ToRectangle};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::AnchorPoint,
    prelude::{PixelColor, Primitive},
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

// Supported themes
pub mod light;

/// Implementation details
// TODO: this should be merged with other widgets
#[macro_export]
macro_rules! slider_style {
    (@option $color_t:ty, None) => {
        None
    };
    (@option $color_t:ty, $color:tt) => {
        Some(<$color_t>::$color)
    };

    (@state $state:ident<$color_t:ty> {
        fill: $fill:tt,
        border: $border:tt,
        background: $background:tt,
        border_thickness: $border_thickness:tt,
        background_thickness: $background_thickness:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::basic::slider::SliderVisualState<$color_t> for $state {
            const FILL_COLOR: Option<$color_t> = $crate::slider_style!(@option $color_t, $fill);
            const BORDER_COLOR: Option<$color_t> = $crate::slider_style!(@option $color_t, $border);
            const BACKGROUND_LINE_COLOR: Option<$color_t> = $crate::slider_style!(@option $color_t, $background);
            const BACKGROUND_LINE_THICKNESS: u32 = $background_thickness;
            const BORDER_THICKNESS: u32 = $border_thickness;
        }
    };

    (@impl $($style:ident<$color_t:ty> {
        direction: $direction:tt,
        thickness: $thickness:tt,
        width: $width:tt,
        states: {
            $($($state:ident),+: $state_desc:tt),+
        }
    }),+) => {
        $(
            #[derive(Default)]
            pub struct $style;
            impl $crate::themes::basic::slider::SliderVisualStyle<$color_t> for $style {
                type Direction = $direction;

                const THICKNESS: u32 = $thickness;
                const WIDTH: u32 = $width;

                paste::paste! {
                    $($(type $state = [<$style $state>];)+)+
                }
            }

            paste::paste! {
                $(
                    $(
                        $crate::slider_style!(@state [<$style $state>]<$color_t> $state_desc);
                   )+
                )+
            }
        )+
    };
}

/// BaseTheme specific binary color slider style helper
#[macro_export]
macro_rules! slider_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            use embedded_gui::widgets::slider::{Horizontal, Vertical};

            $(
                $crate::slider_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color slider style helper
#[macro_export]
macro_rules! slider_style_rgb {
    (@color $mod:ident, $color_t:tt, $($style:ident $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            use embedded_gui::widgets::slider::{Horizontal, Vertical};

            $(
                $crate::slider_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::slider_style_rgb!(@color rgb555, Rgb555, $($style $descriptor)+);
        $crate::slider_style_rgb!(@color rgb565, Rgb565, $($style $descriptor)+);
        $crate::slider_style_rgb!(@color rgb888, Rgb888, $($style $descriptor)+);
    };
}

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

        // TODO verify that this is correct
        let start = bounds.anchor_point(AnchorPoint::CenterLeft);
        let end = bounds.anchor_point(AnchorPoint::CenterRight);

        Line::new(start, end)
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
    pub fn new(visual: VS) -> Self {
        Self {
            visual,
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

pub type StyledSlider<C, S> = Slider<SliderProperties<C, S>, ()>;

pub fn slider<C, S>(range: RangeInclusive<i32>) -> StyledSlider<C::PixelColor, S>
where
    C: BasicTheme,
    S: SliderVisualStyle<C::PixelColor>,
{
    Slider::new(range, SliderProperties::new(S::default()))
}
