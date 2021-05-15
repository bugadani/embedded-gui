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
    alignment::{HorizontalTextAlignment, VerticalTextAlignment},
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
    TextBox as EgTextBox,
};

pub use embedded_text::alignment::{
    BottomAligned, CenterAligned, Justified, LeftAligned, RightAligned, Scrolling, TopAligned,
};

pub mod ascii;
pub mod latin1;

use crate::{EgCanvas, ToRectangle};

pub struct TextBoxStyle<T, H = LeftAligned, V = TopAligned>
where
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    renderer: T,
    horizontal: H,
    vertical: V,
}

impl<'a, 'b, 'c, C, H, V> TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V>
where
    C: PixelColor,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
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
    ) -> TextBoxStyle<MonoTextStyle<'a2, 'b2, 'c2, C>, H, V> {
        TextBoxStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
            horizontal: self.horizontal,
            vertical: self.vertical,
        }
    }
}

impl<F, C, H, V> TextBoxProperties for TextBoxStyle<F, H, V>
where
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    fn measure_text(&self, text: &str, spec: MeasureSpec) -> MeasuredSize {
        let max_width = spec.width.largest().unwrap_or(u32::MAX);
        let max_height = spec.height.largest().unwrap_or(u32::MAX);

        if spec.height.is_exact() {
            return MeasuredSize {
                width: max_width,
                height: spec.height.largest().unwrap(),
            };
        }

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

pub trait TextBoxStyling<'a, 'b, 'c, C, S, T, H, V>: Sized
where
    C: PixelColor,
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    type Color;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color);

    fn text_renderer<T2>(self, renderer: T2) -> TextBox<S, TextBoxStyle<T2, H, V>>
    where
        T2: TextRenderer + CharacterStyle<Color = <T2 as TextRenderer>::Color>;

    fn style<P>(self, props: P) -> TextBox<S, P>
    where
        P: TextBoxProperties;

    fn horizontal_alignment<H2: HorizontalTextAlignment>(
        self,
        alignment: H2,
    ) -> TextBox<S, TextBoxStyle<T, H2, V>>;

    fn vertical_alignment<V2: VerticalTextAlignment>(
        self,
        alignment: V2,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V2>>;
}

impl<'a, 'b, 'c, C, S, H, V> TextBoxStyling<'a, 'b, 'c, C, S, MonoTextStyle<'a, 'b, 'c, C>, H, V>
    for TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V>>
where
    S: AsRef<str>,
    C: PixelColor,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    type Color = C;

    fn set_text_color(&mut self, color: Self::Color) {
        self.label_properties.text_color(color);
    }

    fn text_renderer<T>(self, renderer: T) -> TextBox<S, TextBoxStyle<T, H, V>>
    where
        T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
    {
        let horizontal = self.label_properties.horizontal;
        let vertical = self.label_properties.vertical;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
        })
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

    fn horizontal_alignment<H2: HorizontalTextAlignment>(
        self,
        alignment: H2,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H2, V>> {
        let renderer = self.label_properties.renderer;
        let horizontal = alignment;
        let vertical = self.label_properties.vertical;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
        })
    }

    fn vertical_alignment<V2: VerticalTextAlignment>(
        self,
        alignment: V2,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V2>> {
        let renderer = self.label_properties.renderer;
        let horizontal = self.label_properties.horizontal;
        let vertical = alignment;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
        })
    }
}

/// Font settings specific to `MonoFont`'s renderer.
pub trait MonoFontTextBoxStyling<C, S, H, V>: Sized
where
    S: AsRef<str>,
    C: PixelColor,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    fn font<'a, 'b, 'c>(
        self,
        font: &'a MonoFont<'b, 'c>,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V>>;
}

impl<'a, 'b, 'c, C, S, H, V> MonoFontTextBoxStyling<C, S, H, V>
    for TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>, H, V>>
where
    S: AsRef<str>,
    C: PixelColor,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    fn font<'a2, 'b2, 'c2>(
        self,
        font: &'a2 MonoFont<'b2, 'c2>,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a2, 'b2, 'c2, C>, H, V>> {
        let renderer = MonoTextStyleBuilder::from(&self.label_properties.renderer)
            .font(font)
            .build();
        let horizontal = self.label_properties.horizontal;
        let vertical = self.label_properties.vertical;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
        })
    }
}

impl<S, F, C, DT, H, V> WidgetRenderer<EgCanvas<DT>> for TextBox<S, TextBoxStyle<F, H, V>>
where
    S: AsRef<str>,
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor + From<Rgb888>,
    DT: DrawTarget<Color = C>,
    H: HorizontalTextAlignment,
    V: VerticalTextAlignment,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        EgTextBox::with_textbox_style(
            self.text.as_ref(),
            self.bounds.to_rectangle(),
            self.label_properties.renderer.clone(),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::Exact(VerticalOverdraw::Hidden))
                .alignment(self.label_properties.horizontal)
                .vertical_alignment(self.label_properties.vertical)
                .build(),
        )
        .draw(&mut canvas.target)
        .map(|_| ())
    }
}
