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

use crate::{
    themes::Theme,
    widgets::{
        label::{ascii::LabelConstructor, LabelStyle, LabelStyling},
        primitives::{background::BackgroundStyle, border::BorderStyle},
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
    type Font: MonoFont;

    type Disabled: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    fn font() -> Self::Font;
}

#[allow(type_alias_bounds)]
pub type StyledButton<C, S: ButtonStyle<C>> = Button<
    Background<
        Border<
            FillParent<
                Label<&'static str, LabelStyle<MonoTextStyle<C, S::Font>>>,
                HorizontalAndVertical,
                Center,
                Center,
            >,
            BorderStyle<C>,
        >,
        BackgroundStyle<C>,
    >,
>;
pub fn button<C, S>(label: &'static str) -> StyledButton<C, S>
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
                        .font(S::font())
                        .on_state_changed(|label, state| {
                            label.set_text_color(match state.state() {
                                Button::STATE_DISABLED => S::Disabled::LABEL_COLOR,
                                Button::STATE_HOVERED => S::Hovered::LABEL_COLOR,
                                Button::STATE_PRESSED => S::Pressed::LABEL_COLOR,
                                _ => S::Idle::LABEL_COLOR,
                            });
                        }),
                )
                .align_horizontal(Center)
                .align_vertical(Center),
            )
            .border_color(S::Idle::BORDER_COLOR)
            .on_state_changed(|button, state| {
                button.set_border_color(match state.state() {
                    Button::STATE_DISABLED => S::Disabled::BORDER_COLOR,
                    Button::STATE_HOVERED => S::Hovered::BORDER_COLOR,
                    Button::STATE_PRESSED => S::Pressed::BORDER_COLOR,
                    _ => S::Idle::BORDER_COLOR,
                });
            }),
        )
        .background_color(S::Idle::BACKGROUND_COLOR)
        .on_state_changed(|button, state| {
            button.set_background_color(match state.state() {
                Button::STATE_DISABLED => S::Disabled::BACKGROUND_COLOR,
                Button::STATE_HOVERED => S::Hovered::BACKGROUND_COLOR,
                Button::STATE_PRESSED => S::Pressed::BACKGROUND_COLOR,
                _ => S::Idle::BACKGROUND_COLOR,
            });
        }),
    )
}

pub fn primary_button<C>(label: &'static str) -> StyledButton<C, <C as DefaultTheme>::PrimaryButton>
where
    C: DefaultTheme,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    button::<C, <C as DefaultTheme>::PrimaryButton>(label)
}

pub fn secondary_button<C>(
    label: &'static str,
) -> StyledButton<C, <C as DefaultTheme>::SecondaryButton>
where
    C: DefaultTheme,
    BorderStyle<C>: Default,
    BackgroundStyle<C>: Default,
{
    button::<C, <C as DefaultTheme>::SecondaryButton>(label)
}
