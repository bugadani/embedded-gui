use crate::{
    themes::Theme,
    widgets::{
        label::{ascii::LabelConstructor, LabelStyle, LabelStyling, MonoFontLabelStyling},
        primitives::{background::BackgroundStyle, border::BorderStyle},
    },
};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::PixelColor,
};
use embedded_gui::widgets::{
    button::Button,
    label::Label,
    primitives::{
        background::Background,
        border::Border,
        fill::{Center, FillParent, HorizontalAndVertical},
    },
};

pub mod binary_color;
pub mod rgb;

pub trait DefaultTheme: Theme {
    type PrimaryButton: ButtonStyle<Self>;
    type SecondaryButton: ButtonStyle<Self>;
}

pub trait ButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
}

pub trait ButtonStyle<C: PixelColor> {
    type Disabled: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    const FONT: MonoFont<'static, 'static>;
}

#[allow(type_alias_bounds)]
pub type StyledButton<'a, 'b, 'c, C> = Button<
    Background<
        Border<
            FillParent<
                Label<&'static str, LabelStyle<MonoTextStyle<'a, 'b, 'c, C>>>,
                HorizontalAndVertical,
                Center,
                Center,
            >,
            BorderStyle<C>,
        >,
        BackgroundStyle<C>,
    >,
>;
pub fn button<C, S>(label: &'static str) -> StyledButton<C>
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
                        .text_color(S::Idle::LABEL_COLOR)
                        .font(&S::FONT)
                        .on_state_changed(|label, state| {
                            label.set_text_color(if state.has_state(Button::STATE_DISABLED) {
                                S::Disabled::LABEL_COLOR
                            } else if state.has_state(Button::STATE_HOVERED) {
                                S::Hovered::LABEL_COLOR
                            } else if state.has_state(Button::STATE_PRESSED) {
                                S::Pressed::LABEL_COLOR
                            } else {
                                S::Idle::LABEL_COLOR
                            });
                        }),
                )
                .align_horizontal(Center)
                .align_vertical(Center),
            )
            .border_color(S::Idle::BORDER_COLOR)
            .on_state_changed(|button, state| {
                button.set_border_color(if state.has_state(Button::STATE_DISABLED) {
                    S::Disabled::BORDER_COLOR
                } else if state.has_state(Button::STATE_HOVERED) {
                    S::Hovered::BORDER_COLOR
                } else if state.has_state(Button::STATE_PRESSED) {
                    S::Pressed::BORDER_COLOR
                } else {
                    S::Idle::BORDER_COLOR
                });
            }),
        )
        .background_color(S::Idle::BACKGROUND_COLOR)
        .on_state_changed(|button, state| {
            button.set_background_color(if state.has_state(Button::STATE_DISABLED) {
                S::Disabled::BACKGROUND_COLOR
            } else if state.has_state(Button::STATE_HOVERED) {
                S::Hovered::BACKGROUND_COLOR
            } else if state.has_state(Button::STATE_PRESSED) {
                S::Pressed::BACKGROUND_COLOR
            } else {
                S::Idle::BACKGROUND_COLOR
            });
        }),
    )
}

pub fn primary_button<C>(label: &'static str) -> StyledButton<C>
where
    C: DefaultTheme,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    button::<C, <C as DefaultTheme>::PrimaryButton>(label)
}

pub fn secondary_button<C>(label: &'static str) -> StyledButton<C>
where
    C: DefaultTheme,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    button::<C, <C as DefaultTheme>::SecondaryButton>(label)
}
