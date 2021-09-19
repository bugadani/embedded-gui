//! Helper macros and types to build BaseTheme check boxes

// Themes supported
pub mod light;

use crate::{
    themes::basic::BasicTheme,
    widgets::label::{LabelStyle, LabelStyling, MonoFontLabelStyling},
    EgCanvas, ToRectangle,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoFont, MonoTextStyle},
    prelude::{PixelColor, Primitive},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    state::WidgetState,
    widgets::{
        button::Button,
        graphical::checkbox::{CheckBox, CheckBoxProperties},
        label::Label,
        layouts::linear::{Cell, LinearLayout, Row, WithSpacing},
        toggle::Toggle,
        Widget,
    },
    WidgetRenderer,
};

use object_chain::{Chain, Link};

/// BaseTheme specific binary color check box style helper
#[macro_export]
macro_rules! check_box_style {
    (@state $state:ident<$color_t:ty> {
        label: $label:tt,
        border: $border:tt,
        background: $background:tt,
        check_mark: $check_mark:tt,
    }) => {
        pub struct $state;

        impl $crate::themes::basic::check_box::CheckBoxStateColors<$color_t> for $state {
            const LABEL_COLOR: $color_t = <$color_t>::$label;
            const BORDER_COLOR: $color_t = <$color_t>::$border;
            const BACKGROUND_COLOR: $color_t = <$color_t>::$background;
            const CHECK_MARK_COLOR: $color_t = <$color_t>::$check_mark;
        }
    };

    (@impl $($style:ident<$color_t:ty> {
        font: $font_mod:tt::$font:tt,
        states: {
            $($($state:ident),+: $state_desc:tt),+
        }
    }),+) => {
        $(
            pub struct $style;
            impl $crate::themes::basic::check_box::CheckBoxVisualStyle<$color_t> for $style {
                paste::paste! {
                    $($(type $state = [<$style $state>];)+)+
                }

                const FONT: MonoFont<'static> = mono_font::$font_mod::$font;
            }

            $(
                $(
                    paste::paste! {
                        $crate::check_box_style!(@state [<$style $state>]<$color_t> $state_desc);
                    }
                )+
            )+
        )+
    };
}

/// BaseTheme specific binary color toggle button style helper
#[macro_export]
macro_rules! check_box_style_binary_color {
    ($($style:ident $descriptor:tt),+) => {
        #[allow(unused)]
        pub mod binary_color {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::BinaryColor,
            };

            $(
                $crate::check_box_style!(@impl $style<BinaryColor> $descriptor);
            )+
        }
    };
}

/// BaseTheme specific RGB color toggle button style helper
#[macro_export]
macro_rules! check_box_style_rgb {
    (@color $mod:ident, $color_t:tt, $($style:ident $descriptor:tt)+) => {
        #[allow(unused)]
        pub mod $mod {
            use embedded_graphics::{
                mono_font::{self, MonoFont},
                pixelcolor::$color_t,
                prelude::{RgbColor, WebColors},
            };
            $(
                $crate::check_box_style!(@impl $style<$color_t> $descriptor);
            )+
        }
    };

    ($($style:ident $descriptor:tt),+) => {
        $crate::check_box_style_rgb!(@color rgb555, Rgb555, $($style $descriptor)+);
        $crate::check_box_style_rgb!(@color rgb565, Rgb565, $($style $descriptor)+);
        $crate::check_box_style_rgb!(@color rgb888, Rgb888, $($style $descriptor)+);
    };
}

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
}

pub type StyledCheckBoxDecorator<C, W> = Toggle<
    LinearLayout<Link<Cell<W>, Chain<Cell<CheckBox<CheckBoxStyle<C>>>>>, Row<WithSpacing>>,
    (),
    true,
>;

fn check_box<C, S, W>(inner: W) -> StyledCheckBoxDecorator<C::PixelColor, W>
where
    C: BasicTheme,
    S: CheckBoxVisualStyle<C::PixelColor>,
    W: Widget,
{
    Toggle::new(
        Row::new()
            .spacing(1)
            .add(
                CheckBox::with_style(CheckBoxStyle {
                    background_color: S::Idle::BACKGROUND_COLOR,
                    border_color: S::Idle::BORDER_COLOR,
                    checkmark_color: S::Idle::CHECK_MARK_COLOR,
                    line_width: 1,
                    box_size: 9,
                    is_checked: false,
                })
                .on_state_changed(S::apply_check_box),
            )
            .add(inner),
    )
}

// Type alias to decouple toggle button definition from theme
pub type StyledCheckBox<S, C> =
    StyledCheckBoxDecorator<C, Label<S, LabelStyle<MonoTextStyle<'static, C>>>>;

pub fn styled_check_box<ST, C, S>(label: ST) -> StyledCheckBox<ST, C::PixelColor>
where
    ST: AsRef<str>,
    C: BasicTheme,
    S: CheckBoxVisualStyle<C::PixelColor>,
{
    check_box::<C, S, _>(
        C::label(label)
            .font(&S::FONT)
            .text_color(S::Idle::LABEL_COLOR)
            .on_state_changed(S::apply_label),
    )
}

pub struct CheckBoxStyle<C>
where
    C: PixelColor,
{
    pub background_color: C,
    pub border_color: C,
    pub checkmark_color: C,
    pub line_width: u32,
    pub box_size: u32,
    pub is_checked: bool,
}

impl<C> CheckBoxStyle<C>
where
    C: PixelColor,
{
    fn build_box_style(&self) -> PrimitiveStyle<C> {
        PrimitiveStyleBuilder::new()
            .stroke_alignment(StrokeAlignment::Inside)
            .stroke_color(self.border_color)
            .fill_color(self.background_color)
            .stroke_width(self.line_width)
            .build()
    }

    fn build_check_style(&self) -> PrimitiveStyle<C> {
        PrimitiveStyleBuilder::new()
            .fill_color(self.checkmark_color)
            .stroke_width(0)
            .build()
    }
}

impl<C> CheckBoxProperties for CheckBoxStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn measure(&self, spec: MeasureSpec) -> MeasuredSize {
        MeasuredSize {
            width: spec.width.apply_to_measured(self.box_size),
            height: spec.height.apply_to_measured(self.box_size),
        }
    }

    fn set_border_color(&mut self, color: Self::Color) {
        self.border_color = color;
    }

    fn set_background_color(&mut self, color: Self::Color) {
        self.background_color = color;
    }

    fn set_check_mark_color(&mut self, color: Self::Color) {
        self.checkmark_color = color;
    }

    fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }
}

impl<C, DT> WidgetRenderer<EgCanvas<DT>> for CheckBox<CheckBoxStyle<C>>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    CheckBoxStyle<C>: CheckBoxProperties,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        // TODO: this overdraws. Instead, draw different backgrounds,
        // and draw border without background.
        self.bounding_box()
            .to_rectangle()
            .into_styled(self.checkbox_properties.build_box_style())
            .draw(&mut canvas.target)?;

        if self.checkbox_properties.is_checked {
            let BoundingBox { position, size } = self.bounding_box();
            let space = 2 * self.checkbox_properties.line_width;
            let check_bounds = BoundingBox {
                position: Position {
                    x: position.x + space as i32,
                    y: position.y + space as i32,
                },
                size: MeasuredSize {
                    width: size.width - 2 * space,
                    height: size.height - 2 * space,
                },
            };
            check_bounds
                .to_rectangle()
                .into_styled(self.checkbox_properties.build_check_style())
                .draw(&mut canvas.target)?;
        }

        Ok(())
    }
}
