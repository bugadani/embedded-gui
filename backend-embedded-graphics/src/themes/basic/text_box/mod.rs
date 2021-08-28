//! Helper macros and types to build BaseTheme text boxes

use core::borrow::BorrowMut;

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    prelude::PixelColor,
};
use embedded_gui::{
    geometry::BoundingBox,
    state::WidgetState,
    widgets::{
        text_box::{TextBox, TextBoxFields},
        utils::WidgetDataHolder,
    },
};
use heapless::String;

use crate::{themes::basic::BasicTheme, widgets::text_box::TextBoxStyle as TextBoxStyleStruct};

// Themes supported
pub mod light;

/// BaseTheme specific binary color text box style helper
#[macro_export]
macro_rules! text_box_style {
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
        impl $crate::themes::basic::text_box::TextBoxStyle<$color_t> for $style {
            const TEXT_COLOR: Option<$color_t> = $crate::text_box_style!(@option $color_t, $text);
            const BACKGROUND_COLOR: Option<$color_t> =
                $crate::text_box_style!(@option $color_t, $background);
            const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
        }
    };
}

/// BaseTheme specific BinaryColor color label style helper
#[macro_export]
macro_rules! text_box_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            $(
                $crate::text_box_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color label style helper
#[macro_export]
macro_rules! text_box_style_rgb {
    (@color $mod:ident $color_t:tt $($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::text_box_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::text_box_style_rgb!(@color rgb555 Rgb555 $($style $descriptor),+);
        $crate::text_box_style_rgb!(@color rgb565 Rgb565 $($style $descriptor),+);
        $crate::text_box_style_rgb!(@color rgb888 Rgb888 $($style $descriptor),+);
    };
}

pub trait TextBoxStyle<C: PixelColor> {
    const TEXT_COLOR: Option<C>;
    const BACKGROUND_COLOR: Option<C>;

    const FONT: MonoFont<'static>;

    fn new<S, const N: usize>(
        text: S,
    ) -> TextBox<S, TextBoxStyleStruct<MonoTextStyle<'static, C>>, (), N>
    where
        S: BorrowMut<String<N>>,
    {
        let mut renderer = MonoTextStyleBuilder::new().font(&Self::FONT).build();
        renderer.text_color = Self::TEXT_COLOR;
        renderer.background_color = Self::BACKGROUND_COLOR;

        let mut label_properties = TextBoxStyleStruct::new(renderer);
        if let Some(text_color) = Self::TEXT_COLOR {
            label_properties.cursor_color(text_color);
        }

        TextBox {
            fields: TextBoxFields {
                state: WidgetState::default(),
                parent_index: 0,
                text,
                label_properties,
                bounds: BoundingBox::default(),
                on_text_changed: |_, _| (),
                on_parent_state_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

pub type StyledTextBox<S, C, const N: usize> =
    TextBox<S, TextBoxStyleStruct<MonoTextStyle<'static, C>>, (), N>;

pub fn styled_text_box<ST, C, S, const N: usize>(label: ST) -> StyledTextBox<ST, C::PixelColor, N>
where
    ST: BorrowMut<String<N>>,
    C: BasicTheme,
    S: TextBoxStyle<C::PixelColor>,
{
    S::new(label)
}
