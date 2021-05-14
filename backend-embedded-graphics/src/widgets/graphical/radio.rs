use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Point, Primitive},
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    widgets::{
        graphical::radio::{RadioButton, RadioButtonProperties},
        Widget,
    },
    WidgetRenderer,
};

use crate::{themes::Theme, EgCanvas, ToPoint};

pub struct RadioButtonStyle<C>
where
    C: PixelColor,
{
    pub background_color: C,
    pub border_color: C,
    pub checkmark_color: C,
    pub line_width: u32,
    pub box_size: u32,
    pub is_selected: bool,
}

impl<C> RadioButtonStyle<C>
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

impl<C> Default for RadioButtonStyle<C>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            background_color: C::BACKGROUND_COLOR,
            border_color: C::BORDER_COLOR,
            checkmark_color: C::BORDER_COLOR,
            line_width: 1,
            box_size: 9,
            is_selected: false,
        }
    }
}

impl<C> RadioButtonProperties for RadioButtonStyle<C>
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

    fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
}

impl<C, DT> WidgetRenderer<EgCanvas<DT>> for RadioButton<RadioButtonStyle<C>>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    RadioButtonStyle<C>: RadioButtonProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        // TODO: this overdraws. Instead, draw inside first and border last with transparent fill.
        Circle::new(
            self.bounding_box().position.to_point(),
            self.bounding_box().size.width,
        )
        .into_styled(self.radio_properties.build_box_style())
        .draw(&mut canvas.target)?;

        if self.radio_properties.is_selected {
            let BoundingBox { position, size } = self.bounding_box();
            let space = 2 * self.radio_properties.line_width;
            let check_bounds = Circle::new(
                Point::new(position.x + space as i32, position.y + space as i32),
                size.width - 2 * space,
            );
            check_bounds
                .into_styled(self.radio_properties.build_check_style())
                .draw(&mut canvas.target)?;
        }

        Ok(())
    }
}
