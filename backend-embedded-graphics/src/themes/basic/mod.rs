//! Basic theme implementation.

pub mod button;
pub mod check_box;
pub mod label;
pub mod radio_button;
pub mod scrollbar;
pub mod slider;
pub mod text_block;
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
    text_block::{styled_text_block, StyledTextBlock, TextBlockStyle},
    toggle_button::{
        styled_toggle_button, styled_toggle_button_stretched, StyledToggleButton,
        StyledToggleButtonStretched, ToggleButtonStyle,
    },
};
use embedded_graphics::prelude::PixelColor;

pub trait BasicTheme: Sized {
    type PixelColor: PixelColor;

    type LabelStyle: LabelStyle<Self::PixelColor>;
    type TitleStyle: LabelStyle<Self::PixelColor>;
    type TextBlockStyle: TextBlockStyle<Self::PixelColor>;
    type PrimaryButton: ButtonStyle<Self::PixelColor>;
    type SecondaryButton: ButtonStyle<Self::PixelColor>;
    type ToggleButton: ToggleButtonStyle<Self::PixelColor>;
    type CheckBox: CheckBoxVisualStyle<Self::PixelColor>;
    type RadioButton: RadioButtonVisualStyle<Self::PixelColor>;
    type Slider: SliderVisualStyle<Self::PixelColor>;
    type VerticalScrollbar: ScrollbarVisualStyle<Self::PixelColor>;
    type HorizontalScrollbar: ScrollbarVisualStyle<Self::PixelColor>;

    fn title<S: AsRef<str>>(label: S) -> StyledLabel<S, Self::PixelColor> {
        styled_label::<_, Self, Self::TitleStyle>(label)
    }

    fn label<S: AsRef<str>>(label: S) -> StyledLabel<S, Self::PixelColor> {
        styled_label::<_, Self, Self::LabelStyle>(label)
    }

    fn text_block<S: AsRef<str>>(label: S) -> StyledTextBlock<S, Self::PixelColor> {
        styled_text_block::<_, Self, Self::TextBlockStyle>(label)
    }

    fn primary_button<S: AsRef<str>>(label: S) -> StyledButton<S, Self::PixelColor> {
        styled_button::<_, Self, Self::PrimaryButton>(label)
    }

    fn secondary_button<S: AsRef<str>>(label: S) -> StyledButton<S, Self::PixelColor> {
        styled_button::<_, Self, Self::SecondaryButton>(label)
    }

    fn primary_button_stretched<S: AsRef<str>>(
        label: S,
    ) -> StyledButtonStretched<S, Self::PixelColor> {
        styled_button_stretched::<_, Self, Self::PrimaryButton>(label)
    }

    fn secondary_button_stretched<S: AsRef<str>>(
        label: S,
    ) -> StyledButtonStretched<S, Self::PixelColor> {
        styled_button_stretched::<_, Self, Self::SecondaryButton>(label)
    }

    fn toggle_button<S: AsRef<str>>(label: S) -> StyledToggleButton<S, Self::PixelColor> {
        styled_toggle_button::<_, Self, Self::ToggleButton>(label)
    }

    fn toggle_button_stretched<S: AsRef<str>>(
        label: S,
    ) -> StyledToggleButtonStretched<S, Self::PixelColor> {
        styled_toggle_button_stretched::<_, Self, Self::ToggleButton>(label)
    }

    fn check_box<S: AsRef<str>>(label: S) -> StyledCheckBox<S, Self::PixelColor> {
        styled_check_box::<_, Self, Self::CheckBox>(label)
    }

    fn radio_button<S: AsRef<str>>(label: S) -> StyledRadioButton<S, Self::PixelColor> {
        styled_radio_button::<_, Self, Self::RadioButton>(label)
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
                label::$theme_module::$color_mod::{Label, Title},
                radio_button::$theme_module::$color_mod::RadioButton,
                scrollbar::$theme_module::$color_mod::{HorizontalScrollbar, VerticalScrollbar},
                slider::$theme_module::$color_mod::Slider,
                text_block::$theme_module::$color_mod::TextBlock,
                toggle_button::$theme_module::$color_mod::ToggleButton,
                BasicTheme,
            };

            pub struct $theme;
            impl BasicTheme for LightTheme {
                type PixelColor = $color_t;

                type LabelStyle = Label;
                type TitleStyle = Title;
                type TextBlockStyle = TextBlock;
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
