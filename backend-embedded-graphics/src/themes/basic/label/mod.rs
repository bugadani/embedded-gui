//! Helper macros and types to build BaseTheme labels

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    prelude::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::label::Label};

use crate::{themes::basic::BasicTheme, widgets::label::LabelStyle as LabelStyleStruct};

// Themes supported
pub mod light;

/// BaseTheme specific binary color label style helper
#[macro_export]
macro_rules! label_style {
    (@option $color_t:ty, None) => {
        None
    };
    (@option $color_t:ty, $color:tt) => {
        Some(<$color_t>::$color)
    };

    (@impl $style:ident<$color_t:ty> {
        text: $text:tt,
        background: $background:tt,
        font: $font_mod:tt::$font:tt,
    }) => {
        pub struct $style;
        impl $crate::themes::basic::label::LabelStyle<$color_t> for $style {
            const TEXT_COLOR: Option<$color_t> = $crate::label_style!(@option $color_t, $text);
            const BACKGROUND_COLOR: Option<$color_t> =
                $crate::label_style!(@option $color_t, $background);
            const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
        }
    };
}

/// BaseTheme specific BinaryColor color label style helper
#[macro_export]
macro_rules! label_style_binary_color {
    ($style:ident $descriptor:tt) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            $crate::label_style!(@impl $style<BinaryColor> $descriptor);
        }
    };
}

/// BaseTheme specific RGB color label style helper
#[macro_export]
macro_rules! label_style_rgb {
    (@color $mod:ident::$style:ident<$color_t:tt> $descriptor:tt) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $crate::label_style!(@impl $style<$color_t> $descriptor);
        }
    };

    ($style:ident $descriptor:tt) => {
        $crate::label_style_rgb!(@color rgb555::$style<Rgb555> $descriptor);
        $crate::label_style_rgb!(@color rgb565::$style<Rgb565> $descriptor);
        $crate::label_style_rgb!(@color rgb888::$style<Rgb888> $descriptor);
    };
}

pub trait LabelStyle<C: PixelColor> {
    const TEXT_COLOR: Option<C>;
    const BACKGROUND_COLOR: Option<C>;

    const FONT: MonoFont<'static>;

    fn new<S: AsRef<str>>(text: S) -> Label<S, LabelStyleStruct<MonoTextStyle<'static, C>>> {
        let mut renderer = MonoTextStyleBuilder::new().font(&Self::FONT).build();
        renderer.text_color = Self::TEXT_COLOR;
        renderer.background_color = Self::BACKGROUND_COLOR;

        Label {
            parent_index: 0,
            text,
            label_properties: LabelStyleStruct::new(renderer),
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}

pub type StyledLabel<S, C> = Label<S, LabelStyleStruct<MonoTextStyle<'static, C>>>;

pub fn styled_label<ST, C, S>(label: ST) -> StyledLabel<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: LabelStyle<C::PixelColor>,
{
    S::new(label)
}
