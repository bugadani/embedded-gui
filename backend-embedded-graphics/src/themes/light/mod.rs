use embedded_graphics::prelude::PixelColor;

use crate::themes::light::button::{styled_button, ButtonStyle, StyledButton};

pub mod button;

pub trait BasicTheme: Sized {
    type PixelColor: PixelColor;

    type PrimaryButton: ButtonStyle<Self::PixelColor>;
    type SecondaryButton: ButtonStyle<Self::PixelColor>;

    //fn label(label: &'static str) -> StyledLabel<Self::PixelColor> {
    //    todo!()
    //}

    fn primary_button(label: &'static str) -> StyledButton<Self::PixelColor> {
        styled_button::<Self, Self::PrimaryButton>(label)
    }
    fn secondary_button(label: &'static str) -> StyledButton<Self::PixelColor> {
        styled_button::<Self, Self::SecondaryButton>(label)
    }
}

pub struct DarkTheme;

// TODO simplify this mess - conventions and macros?
pub mod binary_color {
    use embedded_graphics::pixelcolor::BinaryColor;

    use crate::themes::light::{
        button::{
            primary_button::binary_color::PrimaryButton,
            secondary_button::binary_color::SecondaryButton,
        },
        BasicTheme,
    };
    pub struct LightTheme;
    impl BasicTheme for LightTheme {
        type PixelColor = BinaryColor;

        type PrimaryButton = PrimaryButton;
        type SecondaryButton = SecondaryButton;
    }
}

// TODO: so far only rgb888 to reduce clutter
pub mod rgb888 {
    use embedded_graphics::pixelcolor::Rgb888;

    use crate::themes::light::{
        button::{
            primary_button::rgb888::PrimaryButton, secondary_button::rgb888::SecondaryButton,
        },
        BasicTheme,
    };
    pub struct LightTheme;
    impl BasicTheme for LightTheme {
        type PixelColor = Rgb888;

        type PrimaryButton = PrimaryButton;
        type SecondaryButton = SecondaryButton;
    }
}
