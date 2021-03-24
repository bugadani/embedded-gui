use embedded_graphics::{draw_target::DrawTarget, mono_font::{MonoFont, MonoTextStyle}, pixelcolor::PixelColor};
use embedded_gui::widgets::{
    button::Button,
    container::Container,
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
        label::{LabelConstructor, LabelStyle, LabelStyling},
        primitives::{background::BackgroundStyle, border::BorderStyle},
    },
};

pub mod binary_color;
pub mod rgb888;

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

    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    fn font() -> Self::Font;
}

#[allow(type_alias_bounds)]
pub type StyledButton<C, D: DrawTarget, S: ButtonStyle<D::Color>> = Container<
    Button<
        Container<
            Background<
                Container<
                    Border<
                        FillParent<
                            Container<
                                Label<&'static str, LabelStyle<D, MonoTextStyle<C, S::Font>>>,
                            >,
                            HorizontalAndVertical,
                            Center,
                            Center,
                        >,
                        BorderStyle<C>,
                    >,
                >,
                BackgroundStyle<C>,
            >,
        >,
    >,
>;
pub fn button<D, S>(label: &'static str) -> StyledButton<D::Color, D, S>
where
    D: DrawTarget,
    D::Color: DefaultTheme,
    S: ButtonStyle<D::Color>,
    BorderStyle<<D as DrawTarget>::Color>: Default,
    BackgroundStyle<<D as DrawTarget>::Color>: Default,
{
    Button::new(
        Background::new(
            Border::new(
                FillParent::both(
                    Label::new(label)
                        .text_color(S::Idle::LABEL_COLOR)
                        .font(S::font())
                        .on_state_changed(|label, state| {
                            label.label_properties.text_color(match state.state() {
                                Button::STATE_HOVERED => S::Hovered::LABEL_COLOR,
                                Button::STATE_PRESSED => S::Pressed::LABEL_COLOR,
                                _ => S::Idle::LABEL_COLOR,
                            })
                        }),
                )
                .align_horizontal(Center)
                .align_vertical(Center),
            )
            .border_color(S::Idle::BORDER_COLOR)
            .on_state_changed(|button, state| {
                button.border_color(match state.state() {
                    Button::STATE_HOVERED => S::Hovered::BORDER_COLOR,
                    Button::STATE_PRESSED => S::Pressed::BORDER_COLOR,
                    _ => S::Idle::BORDER_COLOR,
                })
            }),
        )
        .background_color(S::Idle::BACKGROUND_COLOR)
        .on_state_changed(|button, state| {
            button.background_color(match state.state() {
                Button::STATE_HOVERED => S::Hovered::BACKGROUND_COLOR,
                Button::STATE_PRESSED => S::Pressed::BACKGROUND_COLOR,
                _ => S::Idle::BACKGROUND_COLOR,
            })
        }),
    )
}

pub fn primary_button<D>(label: &'static str) -> StyledButton<D::Color, D, <D::Color as DefaultTheme>::PrimaryButton>
where
    D: DrawTarget,
    D::Color: DefaultTheme,
    BorderStyle<<D as DrawTarget>::Color>: Default,
    BackgroundStyle<<D as DrawTarget>::Color>: Default,
{
    button::<D, <D::Color as DefaultTheme>::PrimaryButton>(label)
}

pub fn secondary_button<D>(label: &'static str) -> StyledButton<D::Color, D, <D::Color as DefaultTheme>::SecondaryButton>
where
    D: DrawTarget,
    D::Color: DefaultTheme,
    BorderStyle<<D as DrawTarget>::Color>: Default,
    BackgroundStyle<<D as DrawTarget>::Color>: Default,
{
    button::<D, <D::Color as DefaultTheme>::SecondaryButton>(label)
}
