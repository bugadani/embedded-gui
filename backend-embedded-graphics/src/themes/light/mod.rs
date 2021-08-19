use embedded_graphics::prelude::PixelColor;

use crate::themes::light::button::ButtonStyle;

pub trait BasicTheme: Sized {
    type PixelColor: PixelColor;

    type PrimaryButton: ButtonStyle<Self::PixelColor>;
    type SecondaryButton: ButtonStyle<Self::PixelColor>;

    // fn primary_button(label: &'static str) -> StyledButton<Self> {
    //     styled_button::<Self, Self::PrimaryButton>(label)
    // }
    //
    // fn secondary_button(label: &'static str) -> StyledButton<Self> {
    //     styled_button::<Self, Self::SecondaryButton>(label)
    // }
}

pub struct LightTheme;
pub struct DarkTheme;

pub mod button;
