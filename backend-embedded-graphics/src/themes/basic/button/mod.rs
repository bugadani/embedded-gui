//! Helper macros and types to build BaseTheme buttons

// Themes supported
pub mod light;

use crate::{
    themes::basic::BasicTheme,
    widgets::{
        background::BackgroundStyle,
        border::BorderStyle,
        label::{LabelStyle, LabelStyling, MonoFontLabelStyling},
    },
};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::PixelColor,
};
use embedded_gui::{
    state::WidgetState,
    widgets::{
        background::{Background, BackgroundProperties},
        border::{Border, BorderProperties},
        button::Button,
        fill::{Center, FillParent, HorizontalAndVertical},
        label::Label,
        spacing::Spacing,
        Widget,
    },
};

/// Implementation details
// TODO: this should be merged with other widgets
#[macro_export]
macro_rules! button_style {
    (@state $state:ident<$color_t:ty> {
        label: $label:tt,
        border: $border:tt,
        background: $background:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::basic::button::ButtonStateColors<$color_t> for $state {
            const LABEL_COLOR: $color_t = <$color_t>::$label;
            const BORDER_COLOR: $color_t = <$color_t>::$border;
            const BACKGROUND_COLOR: $color_t = <$color_t>::$background;
        }
    };

    (@impl $($style:ident<$color_t:ty, $font_mod:tt::$font:tt> {
        $($state:ident $state_desc:tt),+
    }),+) => {
        $(
            pub struct $style;
            impl $crate::themes::basic::button::ButtonStyle<$color_t> for $style {
                paste::paste! {
                    $(type $state = [<$style $state>];)+
                }

                const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
            }

            $(
                paste::paste! {
                    $crate::button_style!(@state [<$style $state>]<$color_t> $state_desc);
                }
            )+
        )+
    };
}

/// BaseTheme specific binary color button style helper
#[macro_export]
macro_rules! button_style_binary_color {
    ($($style:ident<$font_mod:tt::$font:tt> $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };

            $(
                $crate::button_style!(@impl $style<BinaryColor, $font_mod::$font> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color button style helper
#[macro_export]
macro_rules! button_style_rgb {
    (@color $mod:ident, $color_t:tt, $($style:ident<$font_mod:tt::$font:tt> $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::button_style!(@impl $style<$color_t, $font_mod::$font> $descriptor);
            )+
        }
    };

    ($($style:ident<$font_mod:tt::$font:tt> $descriptor:tt),+) => {
        $crate::button_style_rgb!(@color rgb555, Rgb555, $($style<$font_mod::$font> $descriptor)+);
        $crate::button_style_rgb!(@color rgb565, Rgb565, $($style<$font_mod::$font> $descriptor)+);
        $crate::button_style_rgb!(@color rgb888, Rgb888, $($style<$font_mod::$font> $descriptor)+);
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
        W: Widget,
    {
        background.set_background_color(Self::BACKGROUND_COLOR);
    }

    fn apply_border<W, T>(border: &mut Border<W, T>)
    where
        T: BorderProperties<Color = C>,
        W: Widget,
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
        W: Widget,
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
        W: Widget,
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

pub type StyledButtonDecorator<C, W> =
    Button<Border<Background<W, BackgroundStyle<C>>, BorderStyle<C>>>;

fn button<C, S, W>(inner: W) -> StyledButtonDecorator<C::PixelColor, W>
where
    C: BasicTheme,
    S: ButtonStyle<C::PixelColor>,
    W: Widget,
{
    Button::new(
        Border::with_style(
            Background::with_style(inner, BackgroundStyle::new(S::Idle::BACKGROUND_COLOR))
                .on_state_changed(S::apply_background),
            BorderStyle::new(S::Idle::BORDER_COLOR, 1),
        )
        .on_state_changed(S::apply_border),
    )
}

// Type alias to decouple button definition from theme
pub type StyledButton<'a, C> =
    StyledButtonDecorator<C, Spacing<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>>;

pub fn styled_button<C, S>(label: &'static str) -> StyledButton<C::PixelColor>
where
    C: BasicTheme,
    S: ButtonStyle<C::PixelColor>,
{
    button::<C, S, _>(
        Spacing::new(
            C::label(label)
                .font(&S::FONT)
                .text_color(S::Idle::LABEL_COLOR)
                .on_state_changed(S::apply_label),
        )
        .all(1),
    )
}

pub type StyledButtonStretched<'a, C> = StyledButtonDecorator<
    C,
    FillParent<
        Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>,
        HorizontalAndVertical,
        Center,
        Center,
    >,
>;

pub fn styled_button_stretched<C, S>(label: &'static str) -> StyledButtonStretched<C::PixelColor>
where
    C: BasicTheme,
    S: ButtonStyle<C::PixelColor>,
{
    button::<C, S, _>(
        FillParent::both(
            C::label(label)
                .font(&S::FONT)
                .text_color(S::Idle::LABEL_COLOR)
                .on_state_changed(S::apply_label),
        )
        .align_horizontal(Center)
        .align_vertical(Center),
    )
}
