//! Basic theme implementation.

pub mod button;
pub mod check_box;
pub mod label;
pub mod radio_button;
pub mod scrollbar;
pub mod slider;
pub mod toggle_button;

use core::ops::RangeInclusive;

use crate::themes::basic::{
    button::{
        styled_button, styled_button_stretched, ButtonStyle, StyledButton, StyledButtonStretched,
    },
    check_box::{styled_check_box, CheckBoxVisualStyle, StyledCheckBox},
    label::{styled_label, LabelStyle, StyledLabel},
    radio_button::{styled_radio_button, RadioButtonVisualStyle, StyledRadioButton},
    scrollbar::{
        horizontal_scrollbar, vertical_scrollbar, ScrollbarVisualStyle, StyledHorizontalScrollbar,
        StyledVerticalScrollbar,
    },
    slider::{slider, SliderVisualStyle, StyledSlider},
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
    type CheckBox: CheckBoxVisualStyle<Self::PixelColor>;
    type RadioButton: RadioButtonVisualStyle<Self::PixelColor>;
    type Slider: SliderVisualStyle<Self::PixelColor>;
    type VerticalScrollbar: ScrollbarVisualStyle<Self::PixelColor>;
    type HorizontalScrollbar: ScrollbarVisualStyle<Self::PixelColor>;

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

    fn check_box(label: &'static str) -> StyledCheckBox<Self::PixelColor> {
        styled_check_box::<Self, Self::CheckBox>(label)
    }

    fn radio_button(label: &'static str) -> StyledRadioButton<Self::PixelColor> {
        styled_radio_button::<Self, Self::RadioButton>(label)
    }

    fn horizontal_scrollbar(
    ) -> StyledHorizontalScrollbar<Self::PixelColor, Self::HorizontalScrollbar> {
        horizontal_scrollbar::<Self, Self::HorizontalScrollbar>()
    }

    fn vertical_scrollbar() -> StyledVerticalScrollbar<Self::PixelColor, Self::VerticalScrollbar> {
        vertical_scrollbar::<Self, Self::VerticalScrollbar>()
    }

    fn slider(range: RangeInclusive<i32>) -> StyledSlider<Self::PixelColor, Self::Slider> {
        slider::<Self, Self::Slider>(range)
    }
}

/// This macro is used to define the theme structure.
macro_rules! impl_theme {
    ($theme_module:ident, $theme:ident, $color_mod:ident, $color_t:ident) => {
        pub mod $color_mod {
            use embedded_graphics::pixelcolor::$color_t;

            use $crate::themes::basic::{
                button::{
                    $theme_module::$color_mod::PrimaryButton,
                    $theme_module::$color_mod::SecondaryButton,
                },
                check_box::$theme_module::$color_mod::CheckBox,
                label::$theme_module::$color_mod::Label,
                radio_button::$theme_module::$color_mod::RadioButton,
                scrollbar::$theme_module::$color_mod::{HorizontalScrollbar, VerticalScrollbar},
                slider::$theme_module::$color_mod::Slider,
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
                type CheckBox = CheckBox;
                type RadioButton = RadioButton;
                type Slider = Slider;
                type VerticalScrollbar = VerticalScrollbar;
                type HorizontalScrollbar = HorizontalScrollbar;
            }
        }
    };

    ($theme_module:ident, $theme:ident) => {
        impl_theme!($theme_module, $theme, binary_color, BinaryColor);
        impl_theme!($theme_module, $theme, rgb555, Rgb555);
        impl_theme!($theme_module, $theme, rgb565, Rgb565);
        impl_theme!($theme_module, $theme, rgb888, Rgb888);
    };
}

// Theme definitions
impl_theme!(light, LightTheme);
