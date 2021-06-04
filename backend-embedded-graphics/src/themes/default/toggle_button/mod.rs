use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::PixelColor,
};
use embedded_gui::{
    state::WidgetState,
    widgets::{
        label::Label,
        primitives::{
            background::{Background, BackgroundProperties},
            border::{Border, BorderProperties},
            fill::{Center, FillParent, HorizontalAndVertical},
            spacing::Spacing,
        },
        toggle::Toggle,
    },
};

use crate::{
    themes::default::{button::ButtonStateColors, DefaultTheme},
    widgets::{
        label::{ascii::LabelConstructor, LabelStyle, LabelStyling, MonoFontLabelStyling},
        primitives::{background::BackgroundStyle, border::BorderStyle},
    },
};

pub mod binary_color;
pub mod rgb;

pub trait ToggleButtonStyle<C: PixelColor> {
    type Inactive: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    type InactiveChecked: ButtonStateColors<C>;
    type IdleChecked: ButtonStateColors<C>;
    type HoveredChecked: ButtonStateColors<C>;
    type PressedChecked: ButtonStateColors<C>;

    const FONT: MonoFont<'static>;

    fn apply_label<S, T>(label: &mut Label<S, T>, state: WidgetState)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        if state.has_state(Toggle::STATE_INACTIVE) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::InactiveChecked::apply_label(label);
            } else {
                Self::Inactive::apply_label(label);
            }
        } else if state.has_state(Toggle::STATE_HOVERED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::HoveredChecked::apply_label(label);
            } else {
                Self::Hovered::apply_label(label);
            }
        } else if state.has_state(Toggle::STATE_PRESSED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::PressedChecked::apply_label(label);
            } else {
                Self::Pressed::apply_label(label);
            }
        } else {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::IdleChecked::apply_label(label);
            } else {
                Self::Idle::apply_label(label);
            }
        }
    }

    fn apply_border<W, T>(border: &mut Border<W, T>, state: WidgetState)
    where
        T: BorderProperties<Color = C>,
    {
        if state.has_state(Toggle::STATE_INACTIVE) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::InactiveChecked::apply_border(border);
            } else {
                Self::Inactive::apply_border(border);
            }
        } else if state.has_state(Toggle::STATE_HOVERED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::HoveredChecked::apply_border(border);
            } else {
                Self::Hovered::apply_border(border);
            }
        } else if state.has_state(Toggle::STATE_PRESSED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::PressedChecked::apply_border(border);
            } else {
                Self::Pressed::apply_border(border);
            }
        } else {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::IdleChecked::apply_border(border);
            } else {
                Self::Idle::apply_border(border);
            }
        }
    }

    fn apply_background<W, T>(background: &mut Background<W, T>, state: WidgetState)
    where
        T: BackgroundProperties<Color = C>,
    {
        if state.has_state(Toggle::STATE_INACTIVE) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::InactiveChecked::apply_background(background);
            } else {
                Self::Inactive::apply_background(background);
            }
        } else if state.has_state(Toggle::STATE_HOVERED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::HoveredChecked::apply_background(background);
            } else {
                Self::Hovered::apply_background(background);
            }
        } else if state.has_state(Toggle::STATE_PRESSED) {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::PressedChecked::apply_background(background);
            } else {
                Self::Pressed::apply_background(background);
            }
        } else {
            if state.has_state(Toggle::STATE_CHECKED) {
                Self::IdleChecked::apply_background(background);
            } else {
                Self::Idle::apply_background(background);
            }
        }
    }
}

pub type StyledToggleButton<'a, C> = Toggle<
    Background<
        Border<Spacing<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>, BorderStyle<C>>,
        BackgroundStyle<C>,
    >,
    (),
    true,
>;

pub fn styled_toggle_button<C, S>(label: &'static str) -> StyledToggleButton<C>
where
    C: DefaultTheme,
    S: ToggleButtonStyle<C>,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    Toggle::new(
        Background::new(
            Border::new(
                Spacing::new(
                    Label::new(label)
                        .font(&S::FONT)
                        .on_state_changed(S::apply_label),
                )
                .all(1),
            )
            .on_state_changed(S::apply_border),
        )
        .on_state_changed(S::apply_background),
    )
}

pub type StyledToggleButtonStretched<'a, C> = Toggle<
    Background<
        Border<
            FillParent<
                Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>,
                HorizontalAndVertical,
                Center,
                Center,
            >,
            BorderStyle<C>,
        >,
        BackgroundStyle<C>,
    >,
    (),
    true,
>;

pub fn styled_toggle_button_stretched<C, S>(label: &'static str) -> StyledToggleButtonStretched<C>
where
    C: DefaultTheme,
    S: ToggleButtonStyle<C>,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    Toggle::new(
        Background::new(
            Border::new(
                FillParent::both(
                    Label::new(label)
                        .font(&S::FONT)
                        .on_state_changed(S::apply_label),
                )
                .align_horizontal(Center)
                .align_vertical(Center),
            )
            .on_state_changed(S::apply_border),
        )
        .on_state_changed(S::apply_background),
    )
}
