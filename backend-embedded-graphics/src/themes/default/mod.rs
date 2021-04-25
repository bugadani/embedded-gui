use crate::{
    themes::Theme,
    widgets::{
        graphical::{checkbox::CheckBoxStyle, radio::RadioButtonStyle},
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
    graphical::{
        checkbox::{CheckBox, CheckBoxProperties},
        radio::{RadioButton, RadioButtonProperties},
    },
    label::{Label, LabelProperties},
    layouts::linear::{layout::LinearLayout, row::Row, Cell, Chain, Link, WithSpacing},
    primitives::{
        background::Background,
        border::Border,
        fill::{Center, FillParent, HorizontalAndVertical},
    },
    toggle::Toggle,
};

pub mod binary_color;
pub mod rgb;

pub trait DefaultTheme: Theme {
    type PrimaryButton: ButtonStyle<Self>;
    type SecondaryButton: ButtonStyle<Self>;

    type CheckBox: CheckBoxVisualStyle<Self>;
    type RadioButton: RadioButtonVisualStyle<Self>;
}

pub trait ButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
}

pub trait ButtonStyle<C: PixelColor> {
    type Inactive: ButtonStateColors<C>;
    type Idle: ButtonStateColors<C>;
    type Hovered: ButtonStateColors<C>;
    type Pressed: ButtonStateColors<C>;

    const FONT: MonoFont<'static, 'static>;
}

pub trait CheckBoxStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
    const CHECK_MARK_COLOR: C;

    fn apply_check_box<P: CheckBoxProperties<Color = C>>(check_box: &mut CheckBox<P>) {
        check_box
            .set_background_color(Self::BACKGROUND_COLOR)
            .set_border_color(Self::BORDER_COLOR)
            .set_check_mark_color(Self::CHECK_MARK_COLOR);
    }

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        S: AsRef<str>,
        Label<S, T>: LabelStyling<S, Color = C>,
        T: LabelProperties,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }
}

pub trait CheckBoxVisualStyle<C: PixelColor> {
    type Inactive: CheckBoxStateColors<C>;
    type Idle: CheckBoxStateColors<C>;
    type Hovered: CheckBoxStateColors<C>;
    type Pressed: CheckBoxStateColors<C>;

    const FONT: MonoFont<'static, 'static>;
}

pub trait RadioButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
    const CHECK_MARK_COLOR: C;

    fn apply_radio_button<P: RadioButtonProperties<Color = C>>(radio_button: &mut RadioButton<P>) {
        radio_button
            .set_background_color(Self::BACKGROUND_COLOR)
            .set_border_color(Self::BORDER_COLOR)
            .set_check_mark_color(Self::CHECK_MARK_COLOR);
    }

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        S: AsRef<str>,
        Label<S, T>: LabelStyling<S, Color = C>,
        T: LabelProperties,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }
}

pub trait RadioButtonVisualStyle<C: PixelColor> {
    type Inactive: RadioButtonStateColors<C>;
    type Idle: RadioButtonStateColors<C>;
    type Hovered: RadioButtonStateColors<C>;
    type Pressed: RadioButtonStateColors<C>;

    const FONT: MonoFont<'static, 'static>;
}

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
                            label.set_text_color(if state.has_state(Button::STATE_INACTIVE) {
                                S::Inactive::LABEL_COLOR
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
                button.set_border_color(if state.has_state(Button::STATE_INACTIVE) {
                    S::Inactive::BORDER_COLOR
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
            button.set_background_color(if state.has_state(Button::STATE_INACTIVE) {
                S::Inactive::BACKGROUND_COLOR
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

pub type StyledCheckBox<'a, 'b, 'c, C> = Toggle<
    LinearLayout<
        Link<
            Cell<Label<&'static str, LabelStyle<MonoTextStyle<'a, 'b, 'c, C>>>>,
            Chain<Cell<CheckBox<CheckBoxStyle<C>>>>,
        >,
        Row,
        WithSpacing,
    >,
    (),
    true,
>;

fn styled_checkbox<C, S>(label: &'static str) -> StyledCheckBox<C>
where
    C: DefaultTheme,
    S: CheckBoxVisualStyle<C>,
    CheckBoxStyle<C>: Default,
{
    Toggle::new(
        Row::new(Cell::new(
            CheckBox::<CheckBoxStyle<C>>::new()
                .background_color(S::Idle::BACKGROUND_COLOR)
                .border_color(S::Idle::BORDER_COLOR)
                .check_mark_color(S::Idle::CHECK_MARK_COLOR)
                .on_state_changed(|check_box, state| {
                    check_box.set_checked(state.has_state(Toggle::STATE_CHECKED));

                    if state.has_state(Toggle::STATE_INACTIVE) {
                        S::Inactive::apply_check_box(check_box);
                    } else if state.has_state(Toggle::STATE_HOVERED) {
                        S::Hovered::apply_check_box(check_box);
                    } else if state.has_state(Toggle::STATE_PRESSED) {
                        S::Pressed::apply_check_box(check_box);
                    } else {
                        S::Idle::apply_check_box(check_box);
                    };
                }),
        ))
        .spacing(1)
        .add(Cell::new(
            Label::new(label)
                .text_color(S::Idle::LABEL_COLOR)
                .on_state_changed(|label, state| {
                    if state.has_state(Toggle::STATE_INACTIVE) {
                        S::Inactive::apply_label(label);
                    } else if state.has_state(Toggle::STATE_HOVERED) {
                        S::Hovered::apply_label(label);
                    } else if state.has_state(Toggle::STATE_PRESSED) {
                        S::Pressed::apply_label(label);
                    } else {
                        S::Idle::apply_label(label);
                    };
                }),
        )),
    )
}

pub fn checkbox<C>(label: &'static str) -> StyledCheckBox<C>
where
    C: DefaultTheme,
    CheckBoxStyle<C>: Default,
{
    styled_checkbox::<C, <C as DefaultTheme>::CheckBox>(label)
}

pub type StyledRadioButton<'a, 'b, 'c, C> = Toggle<
    LinearLayout<
        Link<
            Cell<Label<&'static str, LabelStyle<MonoTextStyle<'a, 'b, 'c, C>>>>,
            Chain<Cell<RadioButton<RadioButtonStyle<C>>>>,
        >,
        Row,
        WithSpacing,
    >,
    (),
    false,
>;

fn styled_radio_button<C, S>(label: &'static str) -> StyledRadioButton<C>
where
    C: DefaultTheme,
    S: RadioButtonVisualStyle<C>,
    RadioButtonStyle<C>: Default,
{
    Toggle::new(
        Row::new(Cell::new(
            RadioButton::<RadioButtonStyle<C>>::new()
                .background_color(S::Idle::BACKGROUND_COLOR)
                .border_color(S::Idle::BORDER_COLOR)
                .check_mark_color(S::Idle::CHECK_MARK_COLOR)
                .on_state_changed(|radio_button, state| {
                    radio_button.set_selected(state.has_state(Toggle::STATE_CHECKED));

                    if state.has_state(Toggle::STATE_INACTIVE) {
                        S::Inactive::apply_radio_button(radio_button);
                    } else if state.has_state(Toggle::STATE_HOVERED) {
                        S::Hovered::apply_radio_button(radio_button);
                    } else if state.has_state(Toggle::STATE_PRESSED) {
                        S::Pressed::apply_radio_button(radio_button);
                    } else {
                        S::Idle::apply_radio_button(radio_button);
                    };
                }),
        ))
        .spacing(1)
        .add(Cell::new(
            Label::new(label)
                .text_color(S::Idle::LABEL_COLOR)
                .on_state_changed(|label, state| {
                    if state.has_state(Toggle::STATE_INACTIVE) {
                        S::Inactive::apply_label(label);
                    } else if state.has_state(Toggle::STATE_HOVERED) {
                        S::Hovered::apply_label(label);
                    } else if state.has_state(Toggle::STATE_PRESSED) {
                        S::Pressed::apply_label(label);
                    } else {
                        S::Idle::apply_label(label);
                    };
                }),
        )),
    )
    .disallow_manual_uncheck()
}

pub fn radio_button<C>(label: &'static str) -> StyledRadioButton<C>
where
    C: DefaultTheme,
    RadioButtonStyle<C>: Default,
{
    styled_radio_button::<C, <C as DefaultTheme>::RadioButton>(label)
}
