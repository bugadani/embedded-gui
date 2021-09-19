use core::borrow::BorrowMut;

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
    geometry::{measurement::MeasureSpec, MeasuredSize, Position},
    input::event::{Key, Modifier, ToStr},
    prelude::WidgetData,
    state::selection::Selected,
    widgets::{
        text_box::{TextBox, TextBoxFields, TextBoxProperties},
        utils::WidgetDataHolder,
    },
    WidgetRenderer,
};
use embedded_text::{
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
    TextBox as EgTextBox,
};

pub use embedded_text::alignment::{HorizontalAlignment, VerticalAlignment};
use heapless::String;
use object_chain::ChainElement;

use crate::{widgets::text_box::plugin::Cursor, EgCanvas, ToPoint, ToRectangle};

mod plugin;

pub struct TextBoxStyle<T>
where
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
{
    renderer: T,
    // Temporarily removed as alignments and editor together are a bit buggy
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
    cursor: Cursor,
    cursor_color: Option<<T as TextRenderer>::Color>,
}

impl<C, T> TextBoxStyle<T>
where
    C: PixelColor,
    T: TextRenderer<Color = C> + CharacterStyle<Color = C>,
{
    pub fn new(renderer: T) -> Self {
        Self {
            renderer,
            horizontal: HorizontalAlignment::Left,
            vertical: VerticalAlignment::Top,
            cursor: Cursor::default(),
            cursor_color: None,
        }
    }

    /// Customize the text color
    pub fn text_color(&mut self, text_color: C) {
        self.renderer.set_text_color(Some(text_color));

        if self.cursor_color.is_none() {
            self.cursor_color(text_color);
        }
    }

    /// Customize the cursor color
    pub fn cursor_color(&mut self, color: <T as TextRenderer>::Color) {
        self.cursor_color = Some(color);
    }
}

impl<'a, C> TextBoxStyle<MonoTextStyle<'a, C>>
where
    C: PixelColor,
{
    pub fn get_text_color(&self) -> Option<C> {
        self.renderer.text_color
    }

    /// Customize the font
    pub fn font<'a2>(self, font: &'a2 MonoFont<'a2>) -> TextBoxStyle<MonoTextStyle<'a2, C>> {
        TextBoxStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
            horizontal: self.horizontal,
            vertical: self.vertical,
            cursor: Cursor::default(),
            cursor_color: None,
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

        let bounding_box = EgTextBox::with_textbox_style(
            text,
            Rectangle::new(Point::zero(), Size::new(max_width, max_height)),
            self.renderer.clone(),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::Exact(VerticalOverdraw::Hidden))
                .leading_spaces(true)
                .trailing_spaces(true)
                .build(),
        )
        .bounding_box();

        MeasuredSize {
            width: bounding_box.size.width,
            height: bounding_box.size.height,
        }
    }

    fn handle_keypress<const N: usize>(
        &mut self,
        key: Key,
        modifier: Modifier,
        text: &mut String<N>,
    ) -> bool {
        match key {
            Key::ArrowUp => self.cursor.cursor_up(),
            Key::ArrowDown => self.cursor.cursor_down(),
            Key::ArrowLeft => self.cursor.cursor_left(),
            Key::ArrowRight => self.cursor.cursor_right(),
            Key::Del => self.cursor.delete_after(text),
            Key::Backspace => self.cursor.delete_before(text),
            _ => {
                if let Some(str) = (key, modifier).to_str() {
                    self.cursor.insert(text, str);
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn handle_cursor_down(&mut self, coordinates: Position) {
        self.cursor.move_cursor_to(coordinates.to_point())
    }
}

pub trait TextBoxStyling<B, D, T, const N: usize>: Sized
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
    T: TextRenderer + CharacterStyle<Color = <T as TextRenderer>::Color>,
{
    type Color;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color);

    fn text_renderer<T2>(self, renderer: T2) -> TextBox<B, TextBoxStyle<T2>, D, N>
    where
        T2: TextRenderer + CharacterStyle<Color = <T2 as TextRenderer>::Color>,
        <T2 as TextRenderer>::Color: From<Rgb888>;

    fn style<P>(self, props: P) -> TextBox<B, P, D, N>
    where
        P: TextBoxProperties;

    fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self;

    fn vertical_alignment(self, alignment: VerticalAlignment) -> Self;

    fn cursor_color(self, color: Self::Color) -> Self;
}

impl<'a, B, C, D, T, const N: usize> TextBoxStyling<B, D, T, N>
    for TextBox<B, TextBoxStyle<T>, D, N>
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
    C: PixelColor + From<Rgb888>,
    T: CharacterStyle<Color = C>,
    T: TextRenderer<Color = C>,
{
    type Color = C;

    fn set_text_color(&mut self, color: Self::Color) {
        self.fields.label_properties.text_color(color);
    }

    fn text_renderer<T2>(self, renderer: T2) -> TextBox<B, TextBoxStyle<T2>, D, N>
    where
        T2: TextRenderer + CharacterStyle<Color = <T2 as TextRenderer>::Color>,
        <T2 as TextRenderer>::Color: From<Rgb888>,
    {
        let horizontal = self.fields.label_properties.horizontal;
        let vertical = self.fields.label_properties.vertical;
        let cursor = self.fields.label_properties.cursor.clone();

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
            cursor,
            cursor_color: None, // TODO: convert
        })
    }

    fn style<P>(self, props: P) -> TextBox<B, P, D, N>
    where
        P: TextBoxProperties,
    {
        TextBox {
            fields: TextBoxFields {
                state: self.fields.state,
                parent_index: self.fields.parent_index,
                text: self.fields.text,
                bounds: self.fields.bounds,
                label_properties: props,
                on_text_changed: self.fields.on_text_changed,
                on_parent_state_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::new(self.data_holder.data),
        }
    }

    fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        let renderer = self.fields.label_properties.renderer.clone();
        let horizontal = alignment;
        let vertical = self.fields.label_properties.vertical;
        let cursor = self.fields.label_properties.cursor.clone();
        let cursor_color = self.fields.label_properties.cursor_color;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
            cursor,
            cursor_color,
        })
    }

    fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        let renderer = self.fields.label_properties.renderer.clone();
        let horizontal = self.fields.label_properties.horizontal;
        let vertical = alignment;
        let cursor = self.fields.label_properties.cursor.clone();
        let cursor_color = self.fields.label_properties.cursor_color;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
            cursor,
            cursor_color,
        })
    }

    fn cursor_color(self, color: C) -> Self {
        let renderer = self.fields.label_properties.renderer.clone();
        let horizontal = self.fields.label_properties.horizontal;
        let vertical = self.fields.label_properties.vertical;
        let cursor = self.fields.label_properties.cursor.clone();
        let cursor_color = Some(color);

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
            cursor,
            cursor_color,
        })
    }
}

/// Font settings specific to `MonoFont`'s renderer.
pub trait MonoFontTextBoxStyling<B, C, D, const N: usize>: Sized
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
    C: PixelColor,
{
    fn font<'a>(
        self,
        font: &'a MonoFont<'a>,
    ) -> TextBox<B, TextBoxStyle<MonoTextStyle<'a, C>>, D, N>;
}

impl<'a, B, C, D, const N: usize> MonoFontTextBoxStyling<B, C, D, N>
    for TextBox<B, TextBoxStyle<MonoTextStyle<'a, C>>, D, N>
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
    C: PixelColor + From<Rgb888>,
{
    fn font<'a2>(
        self,
        font: &'a2 MonoFont<'a2>,
    ) -> TextBox<B, TextBoxStyle<MonoTextStyle<'a2, C>>, D, N> {
        let renderer = MonoTextStyleBuilder::from(&self.fields.label_properties.renderer)
            .font(font)
            .build();
        let horizontal = self.fields.label_properties.horizontal;
        let vertical = self.fields.label_properties.vertical;
        let cursor = self.fields.label_properties.cursor.clone();
        let cursor_color = self.fields.label_properties.cursor_color;

        self.style(TextBoxStyle {
            renderer,
            horizontal,
            vertical,
            cursor,
            cursor_color,
        })
    }
}

impl<B, F, C, DT, D, const N: usize> WidgetRenderer<EgCanvas<DT>>
    for TextBox<B, TextBoxStyle<F>, D, N>
where
    B: BorrowMut<String<N>>,
    F: TextRenderer<Color = C> + CharacterStyle<Color = C>,
    C: PixelColor + From<Rgb888>,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        let cursor_color = self.fields.label_properties.cursor_color;

        let textbox = EgTextBox::with_textbox_style(
            self.fields.text.borrow(),
            self.fields.bounds.to_rectangle(),
            self.fields.label_properties.renderer.clone(),
            TextBoxStyleBuilder::new()
                .height_mode(HeightMode::Exact(VerticalOverdraw::Hidden))
                .leading_spaces(true)
                .trailing_spaces(true)
                .alignment(self.fields.label_properties.horizontal)
                .vertical_alignment(self.fields.label_properties.vertical)
                .build(),
        );

        if self.fields.state.has_state(Selected) && cursor_color.is_some() {
            let textbox = textbox.add_plugin(
                self.fields
                    .label_properties
                    .cursor
                    .plugin(cursor_color.unwrap()),
            );

            let result = textbox.draw(&mut canvas.target).map(|_| ());

            let plugins = textbox.take_plugins();
            let (plugin, _plugins) = plugins.pop();
            self.fields.label_properties.cursor = plugin.get_cursor();

            result
        } else {
            textbox.draw(&mut canvas.target).map(|_| ())
        }
    }
}
