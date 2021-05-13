use core::ops::RangeInclusive;

use crate::themes::{
    default::{
        button::{
            primary::{
                binary_color::PrimaryButtonStyle, rgb::PrimaryButtonStyle as RgbPrimaryButtonStyle,
            },
            secondary::{
                binary_color::SecondaryButtonStyle,
                rgb::SecondaryButtonStyle as RgbSecondaryButtonStyle,
            },
            styled_button, ButtonStyle, StyledButton,
        },
        check_box::{
            binary_color::CheckBoxStyle, rgb::CheckBoxStyle as RgbCheckBoxStyle, styled_check_box,
            CheckBoxVisualStyle, StyledCheckBox,
        },
        radio_button::{
            binary_color::RadioButtonStyle, rgb::RadioButtonStyle as RgbRadioButtonStyle,
            styled_radio_button, RadioButtonVisualStyle, StyledRadioButton,
        },
        scrollbar::{
            binary_color::{HorizontalScrollbar, VerticalScrollbar},
            horizontal_scrollbar,
            rgb::{
                HorizontalScrollbar as RgbHorizontalScrollbar,
                VerticalScrollbar as RgbVerticalScrollbar,
            },
            vertical_scrollbar, ScrollbarVisualStyle, StyledHorizontalScrollbar,
            StyledVerticalScrollbar,
        },
        slider::{
            binary_color::SliderStyle, rgb::SliderStyle as RgbSliderStyle, slider,
            SliderVisualStyle, StyledSlider,
        },
    },
    Theme,
};

use embedded_graphics::pixelcolor::{BinaryColor, Rgb555, Rgb565, Rgb888, RgbColor};

pub mod button;
pub mod check_box;
pub mod radio_button;
pub mod scrollbar;
pub mod slider;

pub trait DefaultTheme: Theme {
    type PrimaryButton: ButtonStyle<Self>;
    type SecondaryButton: ButtonStyle<Self>;

    type CheckBox: CheckBoxVisualStyle<Self>;
    type RadioButton: RadioButtonVisualStyle<Self>;

    type Slider: SliderVisualStyle<Self>;
    type VerticalScrollbar: ScrollbarVisualStyle<Self>;
    type HorizontalScrollbar: ScrollbarVisualStyle<Self>;

    fn primary_button(label: &'static str) -> StyledButton<Self> {
        styled_button::<Self, Self::PrimaryButton>(label)
    }

    fn secondary_button(label: &'static str) -> StyledButton<Self> {
        styled_button::<Self, Self::SecondaryButton>(label)
    }

    fn check_box(label: &'static str) -> StyledCheckBox<Self> {
        styled_check_box::<Self, Self::CheckBox>(label)
    }

    fn radio_button(label: &'static str) -> StyledRadioButton<Self> {
        styled_radio_button::<Self, Self::RadioButton>(label)
    }

    fn slider(range: RangeInclusive<i32>) -> StyledSlider<Self> {
        slider::<Self>(range)
    }

    fn vertical_scrollbar() -> StyledVerticalScrollbar<Self> {
        vertical_scrollbar::<Self>()
    }

    fn horizontal_scrollbar() -> StyledHorizontalScrollbar<Self> {
        horizontal_scrollbar::<Self>()
    }
}

impl Theme for BinaryColor {
    const TEXT_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl DefaultTheme for BinaryColor {
    type PrimaryButton = PrimaryButtonStyle;
    type SecondaryButton = SecondaryButtonStyle;

    type CheckBox = CheckBoxStyle;
    type RadioButton = RadioButtonStyle;

    type Slider = SliderStyle;
    type VerticalScrollbar = VerticalScrollbar;
    type HorizontalScrollbar = HorizontalScrollbar;
}

macro_rules! impl_rgb_default_theme {
    ($type:ty) => {
        impl Theme for $type {
            const TEXT_COLOR: Self = Self::BLACK;
            const BORDER_COLOR: Self = Self::BLACK;
            const BACKGROUND_COLOR: Self = Self::WHITE;
        }
        impl DefaultTheme for $type {
            type PrimaryButton = RgbPrimaryButtonStyle<Self>;
            type SecondaryButton = RgbSecondaryButtonStyle<Self>;

            type CheckBox = RgbCheckBoxStyle<Self>;
            type RadioButton = RgbRadioButtonStyle<Self>;

            type Slider = RgbSliderStyle<Self>;
            type VerticalScrollbar = RgbVerticalScrollbar<Self>;
            type HorizontalScrollbar = RgbHorizontalScrollbar<Self>;
        }
    };
}

impl_rgb_default_theme!(Rgb555);
impl_rgb_default_theme!(Rgb565);
impl_rgb_default_theme!(Rgb888);
