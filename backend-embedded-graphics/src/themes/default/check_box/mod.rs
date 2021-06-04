use crate::{
    themes::default::DefaultTheme,
    widgets::{
        graphical::checkbox::CheckBoxStyle,
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
        graphical::checkbox::{CheckBox, CheckBoxProperties},
        label::Label,
        layouts::linear::{layout::LinearLayout, row::Row, Cell, Chain, Link, WithSpacing},
        toggle::Toggle,
    },
};

pub mod binary_color;
pub mod rgb;

pub trait CheckBoxStateColors<C: PixelColor> {
    const LABEL_COLOR: C;
    const BORDER_COLOR: C;
    const BACKGROUND_COLOR: C;
    const CHECK_MARK_COLOR: C;

    fn apply_check_box<P: CheckBoxProperties<Color = C>>(check_box: &mut CheckBox<P>) {
        check_box.set_background_color(Self::BACKGROUND_COLOR);
        check_box.set_border_color(Self::BORDER_COLOR);
        check_box.set_check_mark_color(Self::CHECK_MARK_COLOR);
    }

    fn apply_label<S, T>(label: &mut Label<S, T>)
    where
        Label<S, T>: LabelStyling<S, Color = C>,
    {
        label.set_text_color(Self::LABEL_COLOR);
    }
}

pub trait CheckBoxVisualStyle<C: PixelColor> {
    type Inactive: CheckBoxStateColors<C>;
    type Idle: CheckBoxStateColors<C>;
    type Hovered: CheckBoxStateColors<C>;
    type Pressed: CheckBoxStateColors<C>;

    const FONT: MonoFont<'static>;

    fn apply_check_box<P: CheckBoxProperties<Color = C>>(
        check_box: &mut CheckBox<P>,
        state: WidgetState,
    ) {
        check_box.set_checked(state.has_state(Toggle::STATE_CHECKED));
        if state.has_state(Toggle::STATE_INACTIVE) {
            Self::Inactive::apply_check_box(check_box);
        } else if state.has_state(Toggle::STATE_HOVERED) {
            Self::Hovered::apply_check_box(check_box);
        } else if state.has_state(Toggle::STATE_PRESSED) {
            Self::Pressed::apply_check_box(check_box);
        } else {
            Self::Idle::apply_check_box(check_box);
        };
    }

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
}

pub type StyledCheckBox<'a, 'b, 'c, C> = Toggle<
    LinearLayout<
        Link<
            Cell<Label<&'static str, LabelStyle<MonoTextStyle<'a, C>>>>,
            Chain<Cell<CheckBox<CheckBoxStyle<C>>>>,
        >,
        Row<WithSpacing>,
    >,
    (),
    true,
>;

pub fn styled_check_box<C, S>(label: &'static str) -> StyledCheckBox<C>
where
    C: DefaultTheme,
    S: CheckBoxVisualStyle<C>,
    CheckBoxStyle<C>: Default,
{
    Toggle::new(
        Row::new()
            .spacing(1)
            .add(CheckBox::<CheckBoxStyle<C>>::new().on_state_changed(S::apply_check_box))
            .add(Label::new(label).on_state_changed(S::apply_label)),
    )
}
