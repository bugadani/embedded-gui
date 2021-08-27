use core::marker::PhantomData;

use crate::{themes::basic::BasicTheme, EgCanvas, ToRectangle};
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Primitive},
    primitives::PrimitiveStyle,
    Drawable,
};
use embedded_gui::{
    data::WidgetData,
    widgets::slider::{Slider, SliderDirection, SliderFields, SliderProperties},
    WidgetRenderer,
};

// Supported themes
pub mod light;

/// Implementation details
// TODO: this should be merged with other widgets
#[macro_export]
macro_rules! scrollbar_style {
    (@option $color_t:ty, None) => {
        None
    };
    (@option $color_t:ty, $color:tt) => {
        Some(<$color_t>::$color)
    };

    (@state $state:ident<$color_t:ty> {
        background_fill: $background_fill:tt,
        background_border: $background_border:tt,
        background_border_thickness: $background_border_thickness:tt,
        fill: $fill:tt,
        border: $border:tt,
        border_thickness: $border_thickness:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::basic::scrollbar::ScrollbarVisualState<$color_t> for $state {
            const BACKGROUND_BORDER_COLOR: Option<$color_t> = $crate::scrollbar_style!(@option $color_t, $background_border);
            const BACKGROUND_FILL_COLOR: Option<$color_t> = $crate::scrollbar_style!(@option $color_t, $background_fill);
            const BACKGROUND_BORDER_THICKNESS: u32 = $background_border_thickness;
            const BORDER_COLOR: Option<$color_t> = $crate::scrollbar_style!(@option $color_t, $border);
            const FILL_COLOR: Option<$color_t> = $crate::scrollbar_style!(@option $color_t, $fill);
            const BORDER_THICKNESS: u32 = $border_thickness;
        }
    };

    (@impl_dir $($style:ident<$color_t:ty> {
        direction: $direction:tt,
        thickness: $thickness:tt,
        states: {
            $($state:ident $state_desc:tt),+
        }
    }),+) => {
        $(
            #[derive(Default)]
            pub struct $style;
            impl $crate::themes::basic::scrollbar::ScrollbarVisualStyle<$color_t> for $style {
                type Direction = $direction;

                const THICKNESS: u32 = $thickness;

                paste::paste! {
                    $(type $state = [<$style $state>];)+
                }
            }
            paste::paste! {
                $(
                    $crate::scrollbar_style!(@state [<$style $state>]<$color_t> $state_desc);
                )+
            }
        )+
    };

    (@impl $($style:ident<$color_t:ty> {
        thickness: $thickness:tt,
        states: $state_desc:tt
    }),+) => {
        $(
            paste::paste! {
                $crate::scrollbar_style!(@impl_dir [<Horizontal $style>]<$color_t> {
                    direction: Horizontal,
                    thickness: $thickness,
                    states: $state_desc
                });
                $crate::scrollbar_style!(@impl_dir [<Vertical $style>]<$color_t> {
                    direction: Vertical,
                    thickness: $thickness,
                    states: $state_desc
                });
            }
        )+
    };
}

/// BaseTheme specific binary color scrollbar style helper
#[macro_export]
macro_rules! scrollbar_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            use embedded_gui::widgets::slider::{Horizontal, Vertical};

            $(
                $crate::scrollbar_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color scrollbar style helper
#[macro_export]
macro_rules! scrollbar_style_rgb {
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
                $crate::scrollbar_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::scrollbar_style_rgb!(@color rgb555, Rgb555, $($style $descriptor)+);
        $crate::scrollbar_style_rgb!(@color rgb565, Rgb565, $($style $descriptor)+);
        $crate::scrollbar_style_rgb!(@color rgb888, Rgb888, $($style $descriptor)+);
    };
}

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
    pub fn new(visual: VS) -> Self {
        Self {
            visual,
            window_length: 0,
            _marker: PhantomData,
        }
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

    fn set_length(&mut self, length: u32) {
        self.window_length = length.max(3);
    }
}

impl<VS, C, DT, D> WidgetRenderer<EgCanvas<DT>> for Slider<ScrollbarProperties<C, VS>, D>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    VS: ScrollbarVisualStyle<C>,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.fields.properties.visual.draw(canvas, &self.fields)
    }
}

pub type StyledVerticalScrollbar<C, S> = Slider<ScrollbarProperties<C, S>, ()>;

pub fn vertical_scrollbar<C, S>() -> StyledVerticalScrollbar<C::PixelColor, S>
where
    C: BasicTheme,
    S: ScrollbarVisualStyle<C::PixelColor>,
{
    Slider::new(0..=0, ScrollbarProperties::new(S::default()))
}

pub type StyledHorizontalScrollbar<C, S> = Slider<ScrollbarProperties<C, S>, ()>;

pub fn horizontal_scrollbar<C, S>() -> StyledHorizontalScrollbar<C::PixelColor, S>
where
    C: BasicTheme,
    S: ScrollbarVisualStyle<C::PixelColor>,
{
    Slider::new(0..=0, ScrollbarProperties::new(S::default()))
}
