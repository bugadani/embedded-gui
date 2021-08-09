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
    widgets::text_box::{TextBox, TextBoxProperties},
    WidgetRenderer,
};
use embedded_text::{
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
    TextBox as EgTextBox,
};

pub use embedded_text::alignment::{HorizontalAlignment, VerticalAlignment};

use crate::{EgCanvas, ToRectangle};

pub struct TextBoxStyle<T>
where
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
{
    renderer: T,
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
}

impl<'a, 'b, 'c, C> TextBoxStyle<MonoTextStyle<'a, C>>
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
    pub fn font<'a2>(self, font: &'a2 MonoFont<'a2>) -> TextBoxStyle<MonoTextStyle<'a2, C>> {
        TextBoxStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
            horizontal: self.horizontal,
            vertical: self.vertical,
        }
    }
}

impl<F, C> TextBoxProperties for TextBoxStyle<F>
where
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor + From<Rgb888>,
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

pub trait TextBoxStyling<'a, C, S, T>: Sized
where
    C: PixelColor,
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
{
    type Color;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color);

    fn text_renderer<T2>(self, renderer: T2) -> TextBox<S, TextBoxStyle<T2>>
    where
        T2: TextRenderer + CharacterStyle<Color = <T2 as TextRenderer>::Color>,
        <T2 as TextRenderer>::Color: From<Rgb888>;

    fn style<P>(self, props: P) -> TextBox<S, P>
    where
        P: TextBoxProperties;

    fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self;

    fn vertical_alignment(self, alignment: VerticalAlignment) -> Self;
}

impl<'a, C, S> TextBoxStyling<'a, C, S, MonoTextStyle<'a, C>>
    for TextBox<S, TextBoxStyle<MonoTextStyle<'a, C>>>
where
    S: AsRef<str>,
    C: PixelColor + From<Rgb888>,
{
    type Color = C;

    fn set_text_color(&mut self, color: Self::Color) {
        self.label_properties.text_color(color);
    }

    fn text_renderer<T>(self, renderer: T) -> TextBox<S, TextBoxStyle<T>>
    where
        T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
        <T as TextRenderer>::Color: From<Rgb888>,
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

    fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        let renderer = self.label_properties.renderer;
        let horizontal = alignment;
        let vertical = self.label_properties.vertical;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
        })
    }

    fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
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
pub trait MonoFontTextBoxStyling<C, S>: Sized
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn font<'a>(self, font: &'a MonoFont<'a>) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, C>>>;
}

impl<'a, C, S> MonoFontTextBoxStyling<C, S> for TextBox<S, TextBoxStyle<MonoTextStyle<'a, C>>>
where
    S: AsRef<str>,
    C: PixelColor + From<Rgb888>,
{
    fn font<'a2>(
        self,
        font: &'a2 MonoFont<'a2>,
    ) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a2, C>>> {
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
                .alignment(self.label_properties.horizontal)
                .vertical_alignment(self.label_properties.vertical)
                .build(),
        )
        .draw(&mut canvas.target)
        .map(|_| ())
    }
}

macro_rules! textbox_for_charset {
    ($charset:ident, $font:ident) => {
        pub mod $charset {
            use embedded_graphics::{
                mono_font::{$charset, MonoTextStyle},
                pixelcolor::PixelColor,
            };
            use embedded_gui::{geometry::BoundingBox, widgets::text_box::TextBox};
            use embedded_text::alignment::{HorizontalAlignment, VerticalAlignment};

            use crate::{themes::Theme, widgets::text_box::TextBoxStyle};

            pub trait TextBoxConstructor<'a, S, C>
            where
                S: AsRef<str>,
                C: PixelColor,
            {
                fn new(text: S) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, C>>>;
            }

            impl<'a, 'b, 'c, C, S> TextBoxConstructor<'a, S, C>
                for TextBox<S, TextBoxStyle<MonoTextStyle<'a, C>>>
            where
                S: AsRef<str>,
                C: PixelColor + Theme,
            {
                fn new(text: S) -> Self {
                    TextBox {
                        parent_index: 0,
                        text,
                        label_properties: TextBoxStyle {
                            renderer: MonoTextStyle::new(
                                &$charset::$font,
                                <C as Theme>::TEXT_COLOR,
                            ),
                            horizontal: HorizontalAlignment::Left,
                            vertical: VerticalAlignment::Top,
                        },
                        bounds: BoundingBox::default(),
                        on_state_changed: |_, _| (),
                    }
                }
            }
        }
    };

    ($charset:ident) => {
        textbox_for_charset!($charset, FONT_6X10);
    };
}

textbox_for_charset!(ascii);
textbox_for_charset!(iso_8859_1);
textbox_for_charset!(iso_8859_10);
textbox_for_charset!(iso_8859_13);
textbox_for_charset!(iso_8859_14);
textbox_for_charset!(iso_8859_15);
textbox_for_charset!(iso_8859_16);
textbox_for_charset!(iso_8859_2);
textbox_for_charset!(iso_8859_3);
textbox_for_charset!(iso_8859_4);
textbox_for_charset!(iso_8859_5);
textbox_for_charset!(iso_8859_7);
textbox_for_charset!(iso_8859_9);
textbox_for_charset!(jis_x0201, FONT_6X13);
