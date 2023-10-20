//! Helper macros and types to build BaseTheme text blocks

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    prelude::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::text_block::TextBlock};

use crate::{
    themes::basic::BasicTheme, widgets::text_block::TextBlockStyle as TextBlockStyleStruct,
};

// Themes supported
pub mod light;

/// BaseTheme specific binary color text block style helper
#[macro_export]
macro_rules! text_block_style {
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
        impl $crate::themes::basic::text_block::TextBlockStyle<$color_t> for $style {
            const TEXT_COLOR: Option<$color_t> = $crate::text_block_style!(@option $color_t, $text);
            const BACKGROUND_COLOR: Option<$color_t> =
                $crate::text_block_style!(@option $color_t, $background);
            const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
        }
    };
}

/// BaseTheme specific BinaryColor color label style helper
#[macro_export]
macro_rules! text_block_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            $(
                $crate::text_block_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color label style helper
#[macro_export]
macro_rules! text_block_style_rgb {
    (@color $mod:ident $color_t:tt $($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::text_block_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::text_block_style_rgb!(@color rgb555 Rgb555 $($style $descriptor),+);
        $crate::text_block_style_rgb!(@color rgb565 Rgb565 $($style $descriptor),+);
        $crate::text_block_style_rgb!(@color rgb888 Rgb888 $($style $descriptor),+);
    };
}

pub trait TextBlockStyle<C: PixelColor> {
    const TEXT_COLOR: Option<C>;
    const BACKGROUND_COLOR: Option<C>;

    const FONT: MonoFont<'static>;

    fn new<S: AsRef<str>>(
        text: S,
    ) -> TextBlock<S, TextBlockStyleStruct<MonoTextStyle<'static, C>>> {
        let mut renderer = MonoTextStyleBuilder::new().font(&Self::FONT).build();
        renderer.text_color = Self::TEXT_COLOR;
        renderer.background_color = Self::BACKGROUND_COLOR;

        TextBlock {
            parent_index: 0,
            text,
            label_properties: TextBlockStyleStruct::new(renderer),
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}

pub type StyledTextBlock<S, C> = TextBlock<S, TextBlockStyleStruct<MonoTextStyle<'static, C>>>;

pub fn styled_text_block<ST, C, S>(label: ST) -> StyledTextBlock<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: TextBlockStyle<C::PixelColor>,
{
    S::new(label)
}
