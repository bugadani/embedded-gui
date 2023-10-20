//! Helper macros and types to build BaseTheme radio buttons

// Themes supported
pub mod light;

use crate::{
    themes::basic::BasicTheme,
    widgets::{
        graphical::radio::RadioButtonStyle,
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
        button::Button,
        graphical::radio::{RadioButton, RadioButtonProperties},
        label::Label,
        layouts::linear::{Cell, LinearLayout, Row, WithSpacing},
        toggle::Toggle,
        Widget,
    },
};
use object_chain::{Chain, Link};

/// BaseTheme specific binary color radio button style helper
#[macro_export]
macro_rules! radio_button_style {
    (@state $state:ident<$color_t:ty> {
        label: $label:tt,
        border: $border:tt,
        background: $background:tt,
        check_mark: $check_mark:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::basic::radio_button::RadioButtonStateColors<$color_t> for $state {
            const LABEL_COLOR: $color_t = <$color_t>::$label;
            const BORDER_COLOR: $color_t = <$color_t>::$border;
            const BACKGROUND_COLOR: $color_t = <$color_t>::$background;
            const CHECK_MARK_COLOR: $color_t = <$color_t>::$check_mark;
        }
    };

    (@impl $($style:ident<$color_t:ty> {
        font: $font_mod:tt::$font:tt,
        states: {
            $($($state:ident),+: $state_desc:tt),+
        }
    }),+) => {
        $(
            pub struct $style;
            impl $crate::themes::basic::radio_button::RadioButtonVisualStyle<$color_t> for $style {
                paste::paste! {
                    $($(type $state = [<$style $state>];)+)+
                }

                const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
            }

            $(
                $(
                    paste::paste! {
                        $crate::radio_button_style!(@state [<$style $state>]<$color_t> $state_desc);
                    }
                )+
            )+
        )+
    };
}

/// BaseTheme specific binary color toggle button style helper
#[macro_export]
macro_rules! radio_button_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };

            $(
                $crate::radio_button_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color toggle button style helper
#[macro_export]
macro_rules! radio_button_style_rgb {
    (@color $mod:ident, $color_t:tt, $($style:ident $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::radio_button_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::radio_button_style_rgb!(@color rgb555, Rgb555, $($style $descriptor)+);
        $crate::radio_button_style_rgb!(@color rgb565, Rgb565, $($style $descriptor)+);
        $crate::radio_button_style_rgb!(@color rgb666, Rgb666, $($style $descriptor)+);
        $crate::radio_button_style_rgb!(@color rgb888, Rgb888, $($style $descriptor)+);
    };
}

pub trait RadioButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
    const CHECK_MARK_COLOR: C;

    fn apply_radio_button<P: RadioButtonProperties<Color = C>>(radio_button: &mut RadioButton<P>) {
        radio_button.set_background_color(Self::BACKGROUND_COLOR);
        radio_button.set_border_color(Self::BORDER_COLOR);
        radio_button.set_check_mark_color(Self::CHECK_MARK_COLOR);
    }

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }
}

pub trait RadioButtonVisualStyle<C: PixelColor> {
    type Inactive: RadioButtonStateColors<C>;
    type Idle: RadioButtonStateColors<C>;
    type Hovered: RadioButtonStateColors<C>;
    type Pressed: RadioButtonStateColors<C>;

    const FONT: MonoFont<'static>;

    fn apply_radio_button<P: RadioButtonProperties<Color = C>>(
        radio_button: &mut RadioButton<P>,
        state: WidgetState,
    ) {
        radio_button.set_selected(state.has_state(Toggle::STATE_CHECKED));
        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_radio_button(radio_button);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_radio_button(radio_button);
        } else if state.has_state(Toggle::STATE_PRESSED) {
            Self::Pressed::apply_radio_button(radio_button);
        } else {
            Self::Idle::apply_radio_button(radio_button);
        };
    }

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
}

pub type StyledRadioButtonDecorator<C, W> = Toggle<
    LinearLayout<Link<Cell<W>, Chain<Cell<RadioButton<RadioButtonStyle<C>>>>>, Row<WithSpacing>>,
    (),
    true,
>;

fn radio_button<C, S, W>(inner: W) -> StyledRadioButtonDecorator<C::PixelColor, W>
where
    C: BasicTheme,
    S: RadioButtonVisualStyle<C::PixelColor>,
    W: Widget,
{
    Toggle::new(
        Row::new()
            .spacing(1)
            .add(
                RadioButton::with_style(RadioButtonStyle {
                    background_color: S::Idle::BACKGROUND_COLOR,
                    border_color: S::Idle::BORDER_COLOR,
                    checkmark_color: S::Idle::CHECK_MARK_COLOR,
                    line_width: 1,
                    box_size: 9,
                    is_selected: false,
                })
                .on_state_changed(S::apply_radio_button),
            )
            .add(inner),
    )
}

// Type alias to decouple toggle button definition from theme
pub type StyledRadioButton<S, C> =
    StyledRadioButtonDecorator<C, Label<S, LabelStyle<MonoTextStyle<'static, C>>>>;

pub fn styled_radio_button<ST, C, S>(label: ST) -> StyledRadioButton<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: RadioButtonVisualStyle<C::PixelColor>,
{
    radio_button::<C, S, _>(
        C::label(label)
            .font(&S::FONT)
            .text_color(S::Idle::LABEL_COLOR)
            .on_state_changed(S::apply_label),
    )
}
