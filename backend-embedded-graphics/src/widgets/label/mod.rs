use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
    prelude::Point,
    text::TextRenderer,
};
use embedded_gui::{
    widgets::label::{Label, LabelProperties},
    MeasuredSize, WidgetRenderer,
};

pub mod ascii;
pub mod latin1;

use crate::EgCanvas;

pub struct LabelStyle<T>
where
    T: TextRenderer,
{
    renderer: T,
}

impl<C, F> LabelStyle<MonoTextStyle<C, F>>
where
    F: MonoFont,
    C: PixelColor,
{
    /// Customize the text color
    pub fn text_color(&mut self, text_color: C) {
        self.renderer = MonoTextStyleBuilder::from(&self.renderer)
            .text_color(text_color)
            .build();
    }

    /// Customize the font
    pub fn font<F2: MonoFont>(self, font: F2) -> LabelStyle<MonoTextStyle<C, F2>> {
        LabelStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
        }
    }
}

impl<F, C> LabelProperties for LabelStyle<F>
where
    F: TextRenderer<Color = C>,
    C: PixelColor,
{
    fn measure_text(&self, text: &str) -> MeasuredSize {
        let metrics = self.renderer.measure_string(text, Point::zero());

        MeasuredSize {
            width: metrics.bounding_box.size.width,
            height: metrics.bounding_box.size.height,
        }
    }
}

pub trait LabelStyling<F, C, S>: Sized
where
    S: AsRef<str>,
    F: MonoFont,
    C: PixelColor,
{
    type Color;
    type Font;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color) -> &mut Self;

    fn font<F2: MonoFont>(self, font: F2) -> Label<S, LabelStyle<MonoTextStyle<C, F2>>>;

    fn style<P>(self, props: P) -> Label<S, P>
    where
        P: LabelProperties;
}

impl<F, C, S> LabelStyling<F, C, S> for Label<S, LabelStyle<MonoTextStyle<C, F>>>
where
    S: AsRef<str>,
    F: MonoFont,
    C: PixelColor,
{
    type Color = C;
    type Font = F;

    fn set_text_color(&mut self, color: Self::Color) -> &mut Self {
        self.label_properties.text_color(color);
        self
    }

    fn font<F2: MonoFont>(self, font: F2) -> Label<S, LabelStyle<MonoTextStyle<C, F2>>> {
        let label_properties = self.label_properties.font(font);

        Label {
            parent_index: self.parent_index,
            text: self.text,
            bounds: self.bounds,
            label_properties,
            on_state_changed: |_, _| (),
            state: self.state,
        }
    }

    fn style<P>(self, props: P) -> Label<S, P>
    where
        P: LabelProperties,
    {
        Label {
            parent_index: self.parent_index,
            text: self.text,
            bounds: self.bounds,
            label_properties: props,
            on_state_changed: |_, _| (),
            state: self.state,
        }
    }
}

impl<S, F, C, DT> WidgetRenderer<EgCanvas<DT>> for Label<S, LabelStyle<F>>
where
    S: AsRef<str>,
    F: TextRenderer<Color = C>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.label_properties
            .renderer
            .draw_string(
                self.text.as_ref(),
                Point::new(self.bounds.position.x, self.bounds.position.y),
                &mut canvas.target,
            )
            .map(|_| ())
    }
}
