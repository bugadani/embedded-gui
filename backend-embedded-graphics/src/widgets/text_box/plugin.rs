use core::ops::Sub;

use az::SaturatingAs;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::AnchorPoint,
    prelude::{PixelColor, Point, Primitive},
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::{
        renderer::{CharacterStyle, TextRenderer},
        Baseline,
    },
    Drawable,
};
use embedded_text::{plugin::Plugin, Cursor as RenderingCursor, TextBoxProperties};
use heapless::String;

trait StrExt {
    fn first_n_chars<'a>(&'a self, n: usize) -> &'a str;
}

impl StrExt for str {
    fn first_n_chars<'a>(&'a self, n: usize) -> &'a str {
        if let Some((i, (idx, _))) = self.char_indices().enumerate().take(n + 1).last() {
            if i < n as usize {
                self
            } else {
                &self[0..idx]
            }
        } else {
            self
        }
    }
}

pub(super) struct Cursor {
    /// character offset
    offset: usize,

    /// cursor position in screen coordinates
    pos: Point,

    /// current command
    desired_position: DesiredPosition,

    /// text vertical offset
    vertical_offset: i32,
}

impl Cursor {
    fn plugin<C: PixelColor>(&mut self, color: C) -> EditorPlugin<C> {
        EditorPlugin {
            cursor_position: self.pos,
            current_offset: 0,
            desired_cursor_position: self.desired_position,
            color,
            cursor_drawn: false,
            vertical_offset: self.vertical_offset,
            top_left: Point::zero(),
        }
    }
}

pub(super) struct EditorInput<const N: usize> {
    pub text: String<N>,
    pub cursor: Cursor,
}

impl<const N: usize> EditorInput<N> {
    pub fn new(text: String<N>) -> Self {
        Self {
            cursor: Cursor {
                offset: text.len(),
                pos: Point::zero(),
                desired_position: DesiredPosition::EndOfText,
                vertical_offset: 0,
            },
            text,
        }
    }

    pub fn insert(&mut self, s: &str) {
        //self.text.insert_str(self.cursor.offset, s);
        self.cursor.offset += s.len();
        self.cursor.desired_position = DesiredPosition::Offset(self.cursor.offset);
    }

    pub fn delete_before(&mut self) {
        if self.cursor.offset > 0 {
            self.cursor.offset -= 1;
            self.cursor.desired_position = DesiredPosition::Offset(self.cursor.offset);
            //self.text.remove(self.cursor.offset);
        }
    }

    pub fn delete_after(&mut self) {
        if self.cursor.offset < self.text.chars().count() {
            //self.text.remove(self.cursor.offset);
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.offset > 0 {
            self.cursor.desired_position = DesiredPosition::Offset(self.cursor.offset - 1);
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.offset < self.text.len() {
            self.cursor.desired_position = DesiredPosition::Offset(self.cursor.offset + 1);
        }
    }

    pub fn cursor_up(&mut self) {
        self.cursor.desired_position = DesiredPosition::OneLineUp(
            self.cursor.desired_position.coordinates_or(self.cursor.pos),
        );
    }

    pub fn cursor_down(&mut self) {
        self.cursor.desired_position = DesiredPosition::OneLineDown(
            self.cursor.desired_position.coordinates_or(self.cursor.pos),
        );
    }

    pub fn move_cursor_to(&mut self, point: Point) {
        self.cursor.desired_position = DesiredPosition::ScreenCoordinates(point);
    }
}

#[derive(Clone, Copy, Debug)]
enum DesiredPosition {
    OneLineUp(Point),
    OneLineDown(Point),
    EndOfText,
    Offset(usize),
    /// Move the cursor to the desired text space coordinates
    Coordinates(Point),
    /// Move the cursor to the desired screen space coordinates
    ScreenCoordinates(Point),
}

impl DesiredPosition {
    fn coordinates_or(&self, fallback: Point) -> Point {
        match self {
            DesiredPosition::Coordinates(c) => *c,
            _ => fallback,
        }
    }
}

#[derive(Clone)]
struct EditorPlugin<C> {
    desired_cursor_position: DesiredPosition,
    cursor_position: Point,
    current_offset: usize,
    color: C,
    cursor_drawn: bool,

    /// text vertical offset
    vertical_offset: i32,
    top_left: Point,
}

impl<C: PixelColor> EditorPlugin<C> {
    #[track_caller]
    fn draw_cursor<D>(
        &mut self,
        draw_target: &mut D,
        bounds: Rectangle,
        pos: Point,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let pos = Point::new(pos.x.max(self.top_left.x), pos.y);
        self.cursor_position = self.to_text_space(pos);
        self.cursor_drawn = true;

        let style = PrimitiveStyle::with_stroke(self.color, 1);
        Line::new(
            pos + Point::new(0, 1),
            pos + Point::new(0, bounds.size.height as i32 - 1),
        )
        .into_styled(style)
        .draw(draw_target)
    }

    fn to_text_space(&self, point: Point) -> Point {
        point - Point::new(0, self.vertical_offset) - self.top_left
    }

    fn to_screen_space(&self, point: Point) -> Point {
        point + Point::new(0, self.vertical_offset) + self.top_left
    }

    fn update_cursor(self, cursor: &mut Cursor) {
        cursor.pos = self.cursor_position;
        cursor.offset = self.current_offset;
        cursor.desired_position = self.desired_cursor_position;
        cursor.vertical_offset = self.vertical_offset;
    }
}

impl<'a, C: PixelColor> Plugin<'a, C> for EditorPlugin<C> {
    fn on_start_render<S: CharacterStyle + TextRenderer>(
        &mut self,
        cursor: &mut RenderingCursor,
        props: &TextBoxProperties<'_, S>,
    ) {
        let line_height = props.char_style.line_height() as i32;
        self.top_left = props.bounding_box.top_left;

        self.desired_cursor_position = match self.desired_cursor_position {
            DesiredPosition::OneLineUp(old) => {
                let newy = old.y - line_height;

                if newy < 0 {
                    DesiredPosition::Offset(0)
                } else {
                    DesiredPosition::Coordinates(Point::new(old.x, newy))
                }
            }
            DesiredPosition::OneLineDown(old) => {
                let newy = old.y + line_height;

                if newy >= props.text_height {
                    DesiredPosition::EndOfText
                } else {
                    DesiredPosition::Coordinates(Point::new(old.x, newy))
                }
            }
            DesiredPosition::ScreenCoordinates(point) => {
                let point = self.to_text_space(point);

                if point.y < 0 {
                    DesiredPosition::Offset(0)
                } else if point.y >= props.text_height {
                    DesiredPosition::EndOfText
                } else {
                    DesiredPosition::Coordinates(Point::new(
                        point.x,
                        point.y.min(props.text_height),
                    ))
                }
            }
            pos => pos,
        };

        let cursor_coordinates = self
            .desired_cursor_position
            .coordinates_or(self.cursor_position);

        let cursor_coordinates = self.to_screen_space(cursor_coordinates);

        // Modify current offset value by the amount outside of the current window
        let box_height: i32 = props.bounding_box.size.height.saturating_as();
        let bounds_min = props.bounding_box.top_left.y;
        let bounds_max = bounds_min + box_height;

        self.vertical_offset -= if cursor_coordinates.y < bounds_min {
            cursor_coordinates.y - bounds_min
        } else if cursor_coordinates.y + line_height > bounds_max {
            cursor_coordinates.y + line_height - bounds_max
        } else {
            0
        };

        self.vertical_offset = self
            .vertical_offset
            .max(box_height - props.text_height)
            .min(0);

        cursor.y += self.vertical_offset;

        if let DesiredPosition::Coordinates(pos) = self.desired_cursor_position {
            self.desired_cursor_position =
                DesiredPosition::ScreenCoordinates(self.to_screen_space(pos));
        }
    }

    fn post_render<T, D>(
        &mut self,
        draw_target: &mut D,
        character_style: &T,
        text: Option<&str>,
        bounds: Rectangle,
    ) -> Result<(), D::Error>
    where
        T: TextRenderer<Color = C>,
        D: DrawTarget<Color = T::Color>,
    {
        if self.cursor_drawn {
            return Ok(());
        }

        // Convert different positions to offset
        let len = text.unwrap_or_default().chars().count();
        let desired_cursor_position = match self.desired_cursor_position {
            DesiredPosition::EndOfText => {
                // We only want to draw the cursor, so we don't need to do anything
                // if we are not at the very end of the text
                if text.is_none() {
                    Some(self.current_offset)
                } else {
                    None
                }
            }

            DesiredPosition::ScreenCoordinates(point) => {
                let same_line = point.y >= bounds.top_left.y
                    && point.y <= bounds.anchor_point(AnchorPoint::BottomRight).y;

                if same_line {
                    match text {
                        Some("\n") | None => {
                            // end of text, or cursor is positioned before the text begins
                            Some(self.current_offset)
                        }
                        Some(text) if bounds.anchor_point(AnchorPoint::TopRight).x > point.x => {
                            // Figure out the number of drawn characters, set cursor position
                            // TODO: this can be simplified by iterating over char_indices
                            let mut add = len;
                            let mut anchor_point = bounds.top_left;
                            for i in 0..len {
                                let str_before = text.first_n_chars(i).len();
                                let current_char_offset = text.first_n_chars(i + 1).len();
                                let char_bounds = character_style
                                    .measure_string(
                                        &text[str_before..current_char_offset],
                                        anchor_point,
                                        Baseline::Top,
                                    )
                                    .bounding_box;

                                let top_right = char_bounds.anchor_point(AnchorPoint::TopRight);
                                let top_center = char_bounds.anchor_point(AnchorPoint::TopCenter);

                                if top_center.x > point.x {
                                    add = i;
                                    break;
                                }
                                anchor_point = top_right + Point::new(1, 0);
                            }
                            Some(self.current_offset + add)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }

            DesiredPosition::Offset(desired_offset) => Some(desired_offset),

            other => unreachable!("{:?} should have been replaced in on_start_render", other),
        };

        // Draw cursor
        match desired_cursor_position {
            Some(desired_cursor_position)
                if (self.current_offset..self.current_offset + len.max(1))
                    .contains(&desired_cursor_position) =>
            {
                let chars_before = desired_cursor_position - self.current_offset;
                let pos = if chars_before == 0 {
                    // we want the end of the last character
                    bounds
                        .top_left
                        .sub(Point::new(1, 0))
                        .component_max(self.top_left)
                } else {
                    character_style
                        .measure_string(
                            text.unwrap().first_n_chars(chars_before),
                            bounds.top_left,
                            Baseline::Top,
                        )
                        .bounding_box
                        .anchor_point(AnchorPoint::TopRight)
                };
                self.draw_cursor(draw_target, bounds, pos)?;
                self.current_offset = desired_cursor_position;
            }

            _ => {
                self.current_offset += len;
            }
        }

        Ok(())
    }
}
