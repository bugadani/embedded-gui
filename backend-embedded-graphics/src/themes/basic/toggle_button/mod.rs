//! Helper macros and types to build BaseTheme toggle buttons

// Themes supported
pub mod light;

use crate::{
    themes::basic::{button::ButtonStateColors, BasicTheme},
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
        fill::{Center, FillParent, HorizontalAndVertical},
        label::Label,
        spacing::Spacing,
        toggle::Toggle,
        Widget,
    },
};

/// BaseTheme specific binary color toggle button style helper
#[macro_export]
macro_rules! toggle_button_style {
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

    (@selection_state $selection_state:ident<$color_t:ty> {
        $($($state:ident),+: $state_desc:tt),+
    }) => {

        paste::paste! {
            $($($crate::toggle_button_style!(@state [<$selection_state $state>]<$color_t> $state_desc);)+)+
        }

        pub struct $selection_state;
        impl $crate::themes::basic::toggle_button::ToggleButtonStateStyle<$color_t> for $selection_state {
            paste::paste! {
                $($(type $state = [<$selection_state $state>];)+)+
            }
        }
    };

    (@impl $($style:ident<$color_t:ty> {
        font: $font_mod:tt::$font:tt,
        states: {
            $($($selection_state:ident),+: $selection_state_desc:tt),+
        }
    }),+) => {
        $(
            pub struct $style;
            impl $crate::themes::basic::toggle_button::ToggleButtonStyle<$color_t> for $style {
                paste::paste! {
                    $($(type $selection_state = [<$style $selection_state>];)+)+
                }

                const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
            }

            paste::paste! {
                $(
                    $(
                        $crate::toggle_button_style!(@selection_state [<$style $selection_state>]<$color_t> $selection_state_desc);
                    )+
                )+
            }
        )+
    };
}

/// BaseTheme specific BinaryColor toggle button style helper
#[macro_export]
macro_rules! toggle_button_style_binary_color {
    ($($style:ident $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };
            $(
                $crate::toggle_button_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color toggle button style helper
#[macro_export]
macro_rules! toggle_button_style_rgb {
    (@color $mod:ident, $color_t:tt, $($style:ident $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::toggle_button_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::toggle_button_style_rgb!(@color rgb555, Rgb555, $($style $descriptor)+);
        $crate::toggle_button_style_rgb!(@color rgb565, Rgb565, $($style $descriptor)+);
        $crate::toggle_button_style_rgb!(@color rgb666, Rgb666, $($style $descriptor)+);
        $crate::toggle_button_style_rgb!(@color rgb888, Rgb888, $($style $descriptor)+);
    };
}

pub trait ToggleButtonStateStyle<C: PixelColor> {
    type Inactive: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    fn apply_label<S, T>(label: &mut Label<S, T>, state: WidgetState)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_label(label);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_label(label);
        } else if state.has_state(Toggle::STATE_PRESSED) {
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
        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_border(border);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_border(border);
        } else if state.has_state(Toggle::STATE_PRESSED) {
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
        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_background(background);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_background(background);
        } else if state.has_state(Toggle::STATE_PRESSED) {
            Self::Pressed::apply_background(background);
        } else {
            Self::Idle::apply_background(background);
        };
    }
}

pub trait ToggleButtonStyle<C: PixelColor> {
    type Unchecked: ToggleButtonStateStyle<C>;
    type Checked: ToggleButtonStateStyle<C>;

    const FONT: MonoFont<'static>;

    fn apply_label<S, T>(label: &mut Label<S, T>, state: WidgetState)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        if state.has_state(Toggle::STATE_CHECKED) {
            Self::Checked::apply_label(label, state);
        } else {
            Self::Unchecked::apply_label(label, state);
        };
    }

    fn apply_border<W, T>(border: &mut Border<W, T>, state: WidgetState)
    where
        T: BorderProperties<Color = C>,
        W: Widget,
    {
        if state.has_state(Toggle::STATE_CHECKED) {
            Self::Checked::apply_border(border, state);
        } else {
            Self::Unchecked::apply_border(border, state);
        };
    }

    fn apply_background<W, T>(background: &mut Background<W, T>, state: WidgetState)
    where
        T: BackgroundProperties<Color = C>,
        W: Widget,
    {
        if state.has_state(Toggle::STATE_CHECKED) {
            Self::Checked::apply_background(background, state);
        } else {
            Self::Unchecked::apply_background(background, state);
        };
    }
}

pub type StyledToggleButtonDecorator<C, W> =
    Toggle<Border<Background<W, BackgroundStyle<C>>, BorderStyle<C>>, (), true>;

fn toggle_button<C, S, W>(inner: W) -> StyledToggleButtonDecorator<C::PixelColor, W>
where
    C: BasicTheme,
    S: ToggleButtonStyle<C::PixelColor>,
    W: Widget,
{
    Toggle::new(
        Border::with_style(
            Background::with_style(
                inner,
                BackgroundStyle::new(
                    <<S as ToggleButtonStyle<
                        <C as BasicTheme>::PixelColor>>::Unchecked as ToggleButtonStateStyle<
                            <C as BasicTheme>::PixelColor>>::Idle::BACKGROUND_COLOR),
            )
            .on_state_changed(S::apply_background),
            BorderStyle::new(
                <<S as ToggleButtonStyle<
                    <C as BasicTheme>::PixelColor>>::Unchecked as ToggleButtonStateStyle<
                        <C as BasicTheme>::PixelColor>>::Idle::BORDER_COLOR, 1),
        )
        .on_state_changed(S::apply_border),
    )
}

// Type alias to decouple toggle button definition from theme
pub type StyledToggleButton<S, C> =
    StyledToggleButtonDecorator<C, Spacing<Label<S, LabelStyle<MonoTextStyle<'static, C>>>>>;

pub fn styled_toggle_button<ST, C, S>(label: ST) -> StyledToggleButton<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: ToggleButtonStyle<C::PixelColor>,
{
    toggle_button::<C, S, _>(
        Spacing::new(
            C::label(label)
                .font(&S::FONT)
                .text_color(
                    <<S as ToggleButtonStyle<
                        <C as BasicTheme>::PixelColor>>::Unchecked as ToggleButtonStateStyle<
                            <C as BasicTheme>::PixelColor>>::Idle::LABEL_COLOR)
                .on_state_changed(S::apply_label),
        )
        .all(1),
    )
}

pub type StyledToggleButtonStretched<S, C> = StyledToggleButtonDecorator<
    C,
    FillParent<
        Label<S, LabelStyle<MonoTextStyle<'static, C>>>,
        HorizontalAndVertical,
        Center,
        Center,
    >,
>;

pub fn styled_toggle_button_stretched<ST, C, S>(
    label: ST,
) -> StyledToggleButtonStretched<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: ToggleButtonStyle<C::PixelColor>,
{
    toggle_button::<C, S, _>(
        FillParent::both(
            C::label(label)
                .font(&S::FONT)
                .text_color(
                    <<S as ToggleButtonStyle<
                        <C as BasicTheme>::PixelColor>>::Unchecked as ToggleButtonStateStyle<
                            <C as BasicTheme>::PixelColor>>::Idle::LABEL_COLOR)
                .on_state_changed(S::apply_label),
        )
        .align_horizontal(Center)
        .align_vertical(Center),
    )
}
