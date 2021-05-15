use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{PixelColor, Rgb888},
    prelude::{Dimensions, Point, Size},
    primitives::Rectangle,
    text::renderer::{CharacterStyle, TextRenderer},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, MeasuredSize},
    widgets::textbox::{TextBox, TextBoxProperties},
    WidgetRenderer,
};
use embedded_text::{
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
    TextBox as EgTextBox,
};

pub mod ascii;
pub mod latin1;

use crate::{EgCanvas, ToRectangle};

pub struct TextBoxStyle<T>
where
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
{
    renderer: T,
}

impl<'a, 'b, 'c, C> TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>
where
    C: PixelColor,
{
    /// Customize the text color
    pub fn text_color(&mut self, text_color: C) {
        self.renderer = MonoTextStyleBuilder::from(&self.renderer)
            .text_color(text_color)
            .build();
    }

    /// Customize the font
    pub fn font<'a2, 'b2, 'c2>(
        self,
        font: &'a2 MonoFont<'b2, 'c2>,
    ) -> TextBoxStyle<MonoTextStyle<'a2, 'b2, 'c2, C>> {
        TextBoxStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
        }
    }
}

impl<F, C> TextBoxProperties for TextBoxStyle<F>
where
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor,
{
    fn measure_text(&self, text: &str, spec: MeasureSpec) -> MeasuredSize {
        let max_width = spec.width.largest().unwrap_or(u32::MAX);
        let max_height = spec.height.largest().unwrap_or(u32::MAX);

        let bounding_box = EgTextBox::with_textbox_style(
            text,
            Rectangle::new(Point::zero(), Size::new(max_width, max_height)),
            self.renderer.clone(),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::ShrinkToText(VerticalOverdraw::Hidden))
                .build(),
        )
        .fit_height()
        .bounding_box();

        MeasuredSize {
            width: bounding_box.size.width,
            height: bounding_box.size.height,
        }
    }
}

pub trait TextBoxStyling<S>: Sized {
    type Color;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color);

    fn text_renderer<T>(self, renderer: T) -> TextBox<S, TextBoxStyle<T>>
    where
        T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>;

    fn style<P>(self, props: P) -> TextBox<S, P>
    where
        P: TextBoxProperties;
}

impl<'a, 'b, 'c, C, S> TextBoxStyling<S> for TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>>
where
    S: AsRef<str>,
    C: PixelColor,
{
    type Color = C;

    fn set_text_color(&mut self, color: Self::Color) {
        self.label_properties.text_color(color);
    }

    fn text_renderer<T>(self, renderer: T) -> TextBox<S, TextBoxStyle<T>>
    where
        T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
    {
        self.style(TextBoxStyle { renderer })
    }

    fn style<P>(self, props: P) -> TextBox<S, P>
    where
        P: TextBoxProperties,
    {
        TextBox {
            parent_index: self.parent_index,
            text: self.text,
            bounds: self.bounds,
            label_properties: props,
            on_state_changed: |_, _| (),
        }
    }
}

/// Font settings specific to `MonoFont`'s renderer.
pub trait MonoFontTextBoxStyling<C, S>: Sized
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn font<'a, 'b, 'c>(
        self,
        font: &'a MonoFont<'b, 'c>,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>>;
}

impl<'a, 'b, 'c, C, S> MonoFontTextBoxStyling<C, S>
    for TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>>
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn font<'a2, 'b2, 'c2>(
        self,
        font: &'a2 MonoFont<'b2, 'c2>,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a2, 'b2, 'c2, C>>> {
        let renderer = MonoTextStyleBuilder::from(&self.label_properties.renderer)
            .font(font)
            .build();

        self.style(TextBoxStyle { renderer })
    }
}

impl<S, F, C, DT> WidgetRenderer<EgCanvas<DT>> for TextBox<S, TextBoxStyle<F>>
where
    S: AsRef<str>,
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor + From<Rgb888>,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        EgTextBox::with_textbox_style(
            self.text.as_ref(),
            self.bounds.to_rectangle(),
            self.label_properties.renderer.clone(),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::Exact(VerticalOverdraw::Hidden))
                .build(),
        )
        .draw(&mut canvas.target)
        .map(|_| ())
    }
}
