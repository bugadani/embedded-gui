use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    prelude::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::label::Label};

use crate::{themes::basic::BasicTheme, widgets::label::LabelStyle as LabelStyleStruct};

/// BaseTheme specific binary color label style helper
#[macro_export]
macro_rules! label_style {
    (@option $color_t:ty, None) => {
        None
    };
    (@option $color_t:ty, $color:tt) => {
        Some(<$color_t>::$color)
    };

    ($style:ident<$color_t:ty> {
        text: $text:tt,
        background: $background:tt,
        font: $font:expr,
    }) => {
        pub struct $style;
        impl $crate::themes::basic::label::LabelStyle<$color_t> for $style {
            const TEXT_COLOR: Option<$color_t> = $crate::label_style!(@option $color_t, $text);
            const BACKGROUND_COLOR: Option<$color_t> =
                $crate::label_style!(@option $color_t, $background);
            const FONT: MonoFont<'static> = $font;
        }
    };
}

// TODO this is light theme only
pub mod binary_color {
    use crate::label_style;
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoFont},
        pixelcolor::BinaryColor,
    };

    label_style!(Label<BinaryColor> {
        text: On,
        background: None,
        font: FONT_6X10,
    });
}

/// BaseTheme specific RGB color label style helper
#[macro_export]
macro_rules! label_style_rgb {
    (@color $mod:ident::$style:ident<$color_t:tt> $descriptor:tt) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $crate::label_style!($style<$color_t> $descriptor);
        }
    };

    ($style:ident $descriptor:tt) => {
        $crate::label_style_rgb!(@color rgb555::$style<Rgb555> $descriptor);
        $crate::label_style_rgb!(@color rgb565::$style<Rgb565> $descriptor);
        $crate::label_style_rgb!(@color rgb888::$style<Rgb888> $descriptor);
    };
}

// TODO this is light theme only
label_style_rgb!(Label {
    text: BLACK,
    background: None,
    font: FONT_6X10,
});

pub trait LabelStyle<C: PixelColor> {
    const TEXT_COLOR: Option<C>;
    const BACKGROUND_COLOR: Option<C>;

    const FONT: MonoFont<'static>;

    fn new(text: &'static str) -> Label<&'static str, LabelStyleStruct<MonoTextStyle<'static, C>>> {
        let mut renderer = MonoTextStyleBuilder::new().font(&Self::FONT).build();
        renderer.text_color = Self::TEXT_COLOR;

        Label {
            parent_index: 0,
            text,
            label_properties: LabelStyleStruct::new(renderer),
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}

pub type StyledLabel<'a, C> = Label<&'static str, LabelStyleStruct<MonoTextStyle<'a, C>>>;

pub fn styled_label<C, S>(label: &'static str) -> StyledLabel<C::PixelColor>
where
    C: BasicTheme,
    S: LabelStyle<C::PixelColor>,
{
    S::new(label)
}
