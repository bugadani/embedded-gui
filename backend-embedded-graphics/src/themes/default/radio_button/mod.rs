use crate::{
    themes::default::DefaultTheme,
    widgets::{
        graphical::radio::RadioButtonStyle,
        label::{ascii::LabelConstructor, LabelStyle, LabelStyling},
    },
};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::PixelColor,
};
use embedded_gui::{
    state::WidgetState,
    widgets::{
        graphical::radio::{RadioButton, RadioButtonProperties},
        label::{Label, LabelProperties},
        layouts::linear::{
            object_chain::{Chain, Link},
            Cell, LinearLayout, Row, WithSpacing,
        },
        toggle::Toggle,
    },
};

pub mod binary_color;
pub mod rgb;

pub trait RadioButtonStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
    const CHECK_MARK_COLOR: C;

    fn apply_radio_button<P: RadioButtonProperties<Color = C>>(radio_button: &mut RadioButton<P>) {
        radio_button.set_background_color(Self::BACKGROUND_COLOR);
        radio_button.set_border_color(Self::BORDER_COLOR);
        radio_button.set_check_mark_color(Self::CHECK_MARK_COLOR);
    }

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }
}

pub trait RadioButtonVisualStyle<C: PixelColor> {
    type Inactive: RadioButtonStateColors<C>;
    type Idle: RadioButtonStateColors<C>;
    type Hovered: RadioButtonStateColors<C>;
    type Pressed: RadioButtonStateColors<C>;

    const FONT: MonoFont<'static>;

    fn apply_radio_button<P: RadioButtonProperties<Color = C>>(
        radio_button: &mut RadioButton<P>,
        state: WidgetState,
    ) {
        radio_button.set_selected(state.has_state(Toggle::STATE_CHECKED));

        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_radio_button(radio_button);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_radio_button(radio_button);
        } else if state.has_state(Toggle::STATE_PRESSED) {
            Self::Pressed::apply_radio_button(radio_button);
        } else {
            Self::Idle::apply_radio_button(radio_button);
        };
    }

    fn apply_label<S, T>(label: &mut Label<S, T>, state: WidgetState)
    where
        S: AsRef<str>,
        T: LabelProperties,
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
}

pub type StyledRadioButton<'a, 'b, 'c, C> = Toggle<
    LinearLayout<
        Link<
            Cell<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>,
            Chain<Cell<RadioButton<RadioButtonStyle<C>>>>,
        >,
        Row<WithSpacing>,
    >,
    (),
    false,
>;

pub fn styled_radio_button<C, S>(label: &'static str) -> StyledRadioButton<C>
where
    C: DefaultTheme,
    S: RadioButtonVisualStyle<C>,
    RadioButtonStyle<C>: Default,
{
    Toggle::new(
        Row::new()
            .spacing(1)
            .add(RadioButton::<RadioButtonStyle<C>>::new().on_state_changed(S::apply_radio_button))
            .add(Label::new(label).on_state_changed(S::apply_label)),
    )
    .disallow_manual_uncheck()
}
