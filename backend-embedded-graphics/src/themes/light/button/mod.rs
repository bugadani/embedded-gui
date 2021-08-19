//! Helper macros and types to build BaseTheme buttons

pub mod primary_button;
pub mod secondary_button;

use embedded_graphics::{mono_font::MonoFont, prelude::PixelColor};
use embedded_gui::{
    state::WidgetState,
    widgets::{
        background::{Background, BackgroundProperties},
        border::{Border, BorderProperties},
        button::Button,
        label::Label,
    },
};

use crate::widgets::label::LabelStyling;

/// BaseTheme specific binary color button style helper
#[macro_export]
macro_rules! button_style {
    (@state $state:ident<$color_t:ty> {
        label: $label:tt,
        border: $border:tt,
        background: $background:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::light::button::ButtonStateColors<$color_t> for $state {
            const LABEL_COLOR: $color_t = <$color_t>::$label;
            const BORDER_COLOR: $color_t = <$color_t>::$border;
            const BACKGROUND_COLOR: $color_t = <$color_t>::$background;
        }
    };

    ($style:ident<$color_t:ty, $font:tt> {
        $($state:ident $state_desc:tt),+
    }) => {
        pub struct $style;
        impl $crate::themes::light::button::ButtonStyle<$color_t> for $style {
            $(type $state = $state;)+

            const FONT: MonoFont<'static> = $font;
        }

        $(
            $crate::button_style!(@state $state<$color_t> $state_desc);
        )+
    };
}

/// BaseTheme specific RGB color button style helper
#[macro_export]
macro_rules! button_style_rgb {
    (@color $mod:ident::$style:ident<$color_t:tt, $font:tt> $descriptor:tt) => {
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $crate::button_style!($style<$color_t, $font> $descriptor);
        }
    };

    ($style:ident<$font:tt> $descriptor:tt) => {
        $crate::button_style_rgb!(@color rgb555::$style<Rgb555, $font> $descriptor);
        $crate::button_style_rgb!(@color rgb565::$style<Rgb565, $font> $descriptor);
        $crate::button_style_rgb!(@color rgb888::$style<Rgb888, $font> $descriptor);
    };
}

pub trait ButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }

    fn apply_background<W, T>(background: &mut Background<W, T>)
    where
        T: BackgroundProperties<Color = C>,
    {
        background.set_background_color(Self::BACKGROUND_COLOR);
    }

    fn apply_border<W, T>(border: &mut Border<W, T>)
    where
        T: BorderProperties<Color = C>,
    {
        border.set_border_color(Self::BORDER_COLOR);
    }
}

pub trait ButtonStyle<C: PixelColor> {
    type Inactive: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    const FONT: MonoFont<'static>;

    fn apply_label<S, T>(label: &mut Label<S, T>, state: WidgetState)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        if state.has_state(Button::STATE_INACTIVE) {
            Self::Inactive::apply_label(label);
        } else if state.has_state(Button::STATE_HOVERED) {
            Self::Hovered::apply_label(label);
        } else if state.has_state(Button::STATE_PRESSED) {
            Self::Pressed::apply_label(label);
        } else {
            Self::Idle::apply_label(label);
        };
    }

    fn apply_border<W, T>(border: &mut Border<W, T>, state: WidgetState)
    where
        T: BorderProperties<Color = C>,
    {
        if state.has_state(Button::STATE_INACTIVE) {
            Self::Inactive::apply_border(border);
        } else if state.has_state(Button::STATE_HOVERED) {
            Self::Hovered::apply_border(border);
        } else if state.has_state(Button::STATE_PRESSED) {
            Self::Pressed::apply_border(border);
        } else {
            Self::Idle::apply_border(border);
        };
    }

    fn apply_background<W, T>(background: &mut Background<W, T>, state: WidgetState)
    where
        T: BackgroundProperties<Color = C>,
    {
        if state.has_state(Button::STATE_INACTIVE) {
            Self::Inactive::apply_background(background);
        } else if state.has_state(Button::STATE_HOVERED) {
            Self::Hovered::apply_background(background);
        } else if state.has_state(Button::STATE_PRESSED) {
            Self::Pressed::apply_background(background);
        } else {
            Self::Idle::apply_background(background);
        };
    }
}

use embedded_graphics::mono_font::MonoTextStyle;

use crate::{
    themes::light::BasicTheme,
    widgets::label::{ascii::LabelConstructor, LabelStyle, MonoFontLabelStyling},
};

pub type StyledButton<'a, C> = Button<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>;

pub fn styled_button<C, S>(label: &'static str) -> StyledButton<C::PixelColor>
where
    C: BasicTheme,
    S: ButtonStyle<C::PixelColor>,
{
    // TODO add border, background and spacing
    Button::new(
        C::label(label)
            .font(&S::FONT)
            .on_state_changed(S::apply_label),
    )
}

// TODO: add stretched button once the basics are in place
