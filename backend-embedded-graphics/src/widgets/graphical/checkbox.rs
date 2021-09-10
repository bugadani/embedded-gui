use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Primitive},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    widgets::{
        graphical::checkbox::{CheckBox, CheckBoxProperties},
        Widget,
    },
    WidgetRenderer,
};

use crate::{EgCanvas, ToRectangle};

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
