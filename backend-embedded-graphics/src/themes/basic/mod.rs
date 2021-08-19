use embedded_graphics::prelude::PixelColor;

use crate::themes::basic::button::{styled_button, ButtonStyle, StyledButton};
use crate::themes::basic::label::{styled_label, LabelStyle, StyledLabel};

pub mod button;
pub mod label;

pub trait BasicTheme: Sized {
    type PixelColor: PixelColor;

    type LabelStyle: LabelStyle<Self::PixelColor>;
    type PrimaryButton: ButtonStyle<Self::PixelColor>;
    type SecondaryButton: ButtonStyle<Self::PixelColor>;

    fn label(label: &'static str) -> StyledLabel<Self::PixelColor> {
        styled_label::<Self, Self::LabelStyle>(label)
    }

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

    use crate::themes::basic::{
        button::{
            primary_button::binary_color::PrimaryButton,
            secondary_button::binary_color::SecondaryButton,
        },
        label::binary_color::Label,
        BasicTheme,
    };
    pub struct LightTheme;
    impl BasicTheme for LightTheme {
        type PixelColor = BinaryColor;

        type LabelStyle = Label;
        type PrimaryButton = PrimaryButton;
        type SecondaryButton = SecondaryButton;
    }
}

// TODO: so far only rgb888 to reduce clutter
pub mod rgb888 {
    use embedded_graphics::pixelcolor::Rgb888;

    use crate::themes::basic::{
        button::{
            primary_button::rgb888::PrimaryButton, secondary_button::rgb888::SecondaryButton,
        },
        label::rgb888::Label,
        BasicTheme,
    };
    pub struct LightTheme;
    impl BasicTheme for LightTheme {
        type PixelColor = Rgb888;

        type LabelStyle = Label;
        type PrimaryButton = PrimaryButton;
        type SecondaryButton = SecondaryButton;
    }
}
