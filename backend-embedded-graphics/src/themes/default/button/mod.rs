use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::PixelColor,
};
use embedded_gui::{
    state::WidgetState,
    widgets::{
        background::{Background, BackgroundProperties},
        border::{Border, BorderProperties},
        button::Button,
        fill::{Center, FillParent, HorizontalAndVertical},
        label::Label,
        spacing::Spacing,
        Widget,
    },
};

use crate::{
    themes::default::DefaultTheme,
    widgets::{
        background::BackgroundStyle,
        border::BorderStyle,
        label::{ascii::LabelConstructor, LabelStyle, LabelStyling, MonoFontLabelStyling},
    },
};

pub mod primary;
pub mod secondary;

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
        W: Widget,
    {
        background.set_background_color(Self::BACKGROUND_COLOR);
    }

    fn apply_border<W, T>(border: &mut Border<W, T>)
    where
        T: BorderProperties<Color = C>,
        W: Widget,
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
        W: Widget,
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
        W: Widget,
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

pub type StyledButton<'a, C> = Button<
    Background<
        Border<Spacing<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>, BorderStyle<C>>,
        BackgroundStyle<C>,
    >,
>;

pub fn styled_button<C, S>(label: &'static str) -> StyledButton<C>
where
    C: DefaultTheme,
    S: ButtonStyle<C>,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    Button::new(
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

pub type StyledButtonStretched<'a, C> = Button<
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
>;

pub fn styled_button_stretched<C, S>(label: &'static str) -> StyledButtonStretched<C>
where
    C: DefaultTheme,
    S: ButtonStyle<C>,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    Button::new(
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
