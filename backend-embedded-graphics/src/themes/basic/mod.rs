//! Basic theme implementation.

pub mod button;
pub mod label;
pub mod toggle_button;

use crate::themes::basic::{
    button::{
        styled_button, styled_button_stretched, ButtonStyle, StyledButton, StyledButtonStretched,
    },
    label::{styled_label, LabelStyle, StyledLabel},
    toggle_button::{
        styled_toggle_button, styled_toggle_button_stretched, StyledToggleButton,
        StyledToggleButtonStretched, ToggleButtonStyle,
    },
};
use embedded_graphics::prelude::PixelColor;

pub trait BasicTheme: Sized {
    type PixelColor: PixelColor;

    type LabelStyle: LabelStyle<Self::PixelColor>;
    type PrimaryButton: ButtonStyle<Self::PixelColor>;
    type SecondaryButton: ButtonStyle<Self::PixelColor>;
    type ToggleButton: ToggleButtonStyle<Self::PixelColor>;

    fn label<S: AsRef<str>>(label: S) -> StyledLabel<S, Self::PixelColor> {
        styled_label::<Self, Self::LabelStyle, _>(label)
    }

    fn primary_button(label: &'static str) -> StyledButton<Self::PixelColor> {
        styled_button::<Self, Self::PrimaryButton>(label)
    }

    fn secondary_button(label: &'static str) -> StyledButton<Self::PixelColor> {
        styled_button::<Self, Self::SecondaryButton>(label)
    }

    fn primary_button_stretched(label: &'static str) -> StyledButtonStretched<Self::PixelColor> {
        styled_button_stretched::<Self, Self::PrimaryButton>(label)
    }

    fn secondary_button_stretched(label: &'static str) -> StyledButtonStretched<Self::PixelColor> {
        styled_button_stretched::<Self, Self::SecondaryButton>(label)
    }

    fn toggle_button(label: &'static str) -> StyledToggleButton<Self::PixelColor> {
        styled_toggle_button::<Self, Self::ToggleButton>(label)
    }

    fn toggle_button_stretched(
        label: &'static str,
    ) -> StyledToggleButtonStretched<Self::PixelColor> {
        styled_toggle_button_stretched::<Self, Self::ToggleButton>(label)
    }
}

/// This macro is used to define the theme structure.
macro_rules! impl_theme {
    (@impl $theme_module:ident, $theme:ident, $color_mod:ident, $color_t:ident) => {
        pub mod $color_mod {
            use embedded_graphics::pixelcolor::$color_t;

            use $crate::themes::basic::{
                button::{
                    $theme_module::$color_mod::PrimaryButton,
                    $theme_module::$color_mod::SecondaryButton,
                },
                label::$theme_module::$color_mod::Label,
                toggle_button::$theme_module::$color_mod::ToggleButton,
                BasicTheme,
            };

            pub struct $theme;
            impl BasicTheme for LightTheme {
                type PixelColor = $color_t;

                type LabelStyle = Label;
                type PrimaryButton = PrimaryButton;
                type SecondaryButton = SecondaryButton;
                type ToggleButton = ToggleButton;
            }
        }
    };

    ($theme_module:ident, $theme:ident) => {
        impl_theme!(@impl $theme_module, $theme, binary_color, BinaryColor);
        impl_theme!(@impl $theme_module, $theme, rgb555, Rgb555);
        impl_theme!(@impl $theme_module, $theme, rgb565, Rgb565);
        impl_theme!(@impl $theme_module, $theme, rgb888, Rgb888);
    };
}

// Theme definitions
impl_theme!(light, LightTheme);
