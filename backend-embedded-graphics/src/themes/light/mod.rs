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

/// BaseTheme specific binary color button style helper
macro_rules! button_style {
    (@state $state:ident<$color_t:ty> {
        label: $label:tt,
        border: $border:tt,
        background: $background:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::light::ButtonStateColors<$color_t> for $state {
            const LABEL_COLOR: $color_t = <$color_t>::$label;
            const BORDER_COLOR: $color_t = <$color_t>::$border;
            const BACKGROUND_COLOR: $color_t = <$color_t>::$background;
        }
    };

    ($style:ident<$color_t:ty, $font:tt> {
        $($state:ident $state_desc:tt),+
    }) => {
        pub struct $style;
        impl $crate::themes::light::ButtonStyle<$color_t> for $style {
            $(type $state = $state;)+

            const FONT: MonoFont<'static> = $font;
        }

        $(
            button_style!(@state $state<$color_t> $state_desc);
        )+
    };
}

/// BaseTheme specific RGB color button style helper
macro_rules! button_style_rgb {
    (@color $mod:ident::$style:ident<$color_t:tt, $font:tt> $descriptor:tt) => {
        mod $mod {
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            button_style!($style<$color_t, $font> $descriptor);
        }
    };

    ($style:ident<$font:tt> $descriptor:tt) => {
        button_style_rgb!(@color rgb555::$style<Rgb555, $font> $descriptor);
        button_style_rgb!(@color rgb565::$style<Rgb565, $font> $descriptor);
        button_style_rgb!(@color rgb888::$style<Rgb888, $font> $descriptor);
    };
}

mod binary_color {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoFont},
        pixelcolor::BinaryColor,
    };

    button_style!(PrimaryButton<BinaryColor, FONT_6X10> {
        Inactive {
            label: Off,
            border: On,
            background: On,
        },
        Idle {
            label: Off,
            border: On,
            background: On,
        },
        Hovered {
            label: On,
            border: On,
            background: Off,
        },
        Pressed {
            label: Off,
            border: On,
            background: On,
        }
    });
}

button_style_rgb!(
    PrimaryButton<FONT_6X10> {
        Inactive {
            label: CSS_LIGHT_GRAY,
            border: CSS_DIM_GRAY,
            background: CSS_DIM_GRAY,
        },
        Idle {
            label: WHITE,
            border: CSS_STEEL_BLUE,
            background: CSS_STEEL_BLUE,
        },
        Hovered {
            label: WHITE,
            border: CSS_DODGER_BLUE,
            background: CSS_DODGER_BLUE,
        },
        Pressed {
            label: WHITE,
            border: CSS_LIGHT_STEEL_BLUE,
            background: CSS_LIGHT_STEEL_BLUE,
        }
    }
);
