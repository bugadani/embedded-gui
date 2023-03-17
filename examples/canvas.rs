use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
    widgets::canvas::{Canvas, CanvasStyle},
    EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{RgbColor, Size as EgSize},
    primitives::{Circle, Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_graphics_simulator::{
    sdl2::{Keycode, Mod, MouseButton},
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, Key, KeyEvent, Modifier, PointerEvent},
    prelude::*,
    widgets::{border::Border, fill::FillParent, layouts::linear::Column},
};

trait Convert {
    type Output;

    fn convert(self) -> Self::Output;
}

impl Convert for Keycode {
    type Output = Option<Key>;

    fn convert(self) -> Self::Output {
        match self {
            Keycode::Backspace => Some(Key::Backspace),
            Keycode::Tab => Some(Key::Tab),
            Keycode::Return => Some(Key::Enter),
            Keycode::Space => Some(Key::Space),
            Keycode::KpComma | Keycode::Comma => Some(Key::Comma),
            Keycode::KpMinus | Keycode::Minus => Some(Key::Minus),
            Keycode::KpPeriod | Keycode::Period => Some(Key::Period),
            Keycode::Kp1 | Keycode::Num0 => Some(Key::N0),
            Keycode::Kp2 | Keycode::Num1 => Some(Key::N1),
            Keycode::Kp3 | Keycode::Num2 => Some(Key::N2),
            Keycode::Kp4 | Keycode::Num3 => Some(Key::N3),
            Keycode::Kp5 | Keycode::Num4 => Some(Key::N4),
            Keycode::Kp6 | Keycode::Num5 => Some(Key::N5),
            Keycode::Kp7 | Keycode::Num6 => Some(Key::N6),
            Keycode::Kp8 | Keycode::Num7 => Some(Key::N7),
            Keycode::Kp9 | Keycode::Num8 => Some(Key::N8),
            Keycode::Kp0 | Keycode::Num9 => Some(Key::N9),
            Keycode::A => Some(Key::A),
            Keycode::B => Some(Key::B),
            Keycode::C => Some(Key::C),
            Keycode::D => Some(Key::D),
            Keycode::E => Some(Key::E),
            Keycode::F => Some(Key::F),
            Keycode::G => Some(Key::G),
            Keycode::H => Some(Key::H),
            Keycode::I => Some(Key::I),
            Keycode::J => Some(Key::J),
            Keycode::K => Some(Key::K),
            Keycode::L => Some(Key::L),
            Keycode::M => Some(Key::M),
            Keycode::N => Some(Key::N),
            Keycode::O => Some(Key::O),
            Keycode::P => Some(Key::P),
            Keycode::Q => Some(Key::Q),
            Keycode::R => Some(Key::R),
            Keycode::S => Some(Key::S),
            Keycode::T => Some(Key::T),
            Keycode::U => Some(Key::U),
            Keycode::V => Some(Key::V),
            Keycode::W => Some(Key::W),
            Keycode::X => Some(Key::X),
            Keycode::Y => Some(Key::Y),
            Keycode::Z => Some(Key::Z),
            Keycode::Delete => Some(Key::Del),
            Keycode::Right => Some(Key::ArrowRight),
            Keycode::Left => Some(Key::ArrowLeft),
            Keycode::Down => Some(Key::ArrowDown),
            Keycode::Up => Some(Key::ArrowUp),
            _ => None,
        }
    }
}

impl Convert for Mod {
    type Output = Modifier;

    fn convert(self) -> Self::Output {
        if self.contains(Mod::RALTMOD) {
            Modifier::Alt
        } else if self.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) {
            Modifier::Shift
        } else if self.contains(Mod::CAPSMOD) {
            Modifier::Shift
        } else {
            Modifier::None
        }
    }
}

fn convert_input(event: SimulatorEvent) -> Result<InputEvent, bool> {
    unsafe {
        // This is fine for a demo
        static mut MOUSE_DOWN: bool = false;
        match event {
            SimulatorEvent::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                point,
            } => {
                MOUSE_DOWN = false;
                Ok(InputEvent::PointerEvent(
                    Position {
                        x: point.x,
                        y: point.y,
                    },
                    PointerEvent::Up,
                ))
            }
            SimulatorEvent::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                point,
            } => {
                MOUSE_DOWN = true;
                Ok(InputEvent::PointerEvent(
                    Position {
                        x: point.x,
                        y: point.y,
                    },
                    PointerEvent::Down,
                ))
            }
            SimulatorEvent::MouseMove { point } => Ok(InputEvent::PointerEvent(
                Position {
                    x: point.x,
                    y: point.y,
                },
                if MOUSE_DOWN {
                    PointerEvent::Drag
                } else {
                    PointerEvent::Hover
                },
            )),
            SimulatorEvent::KeyDown {
                keycode, keymod, ..
            } => Ok(InputEvent::KeyEvent(KeyEvent::KeyDown(
                keycode.convert().ok_or(false)?,
                keymod.convert(),
                0,
            ))),

            SimulatorEvent::KeyUp {
                keycode, keymod, ..
            } => Ok(InputEvent::KeyEvent(KeyEvent::KeyUp(
                keycode.convert().ok_or(false)?,
                keymod.convert(),
            ))),
            SimulatorEvent::Quit => Err(true),
            _ => Err(false),
        }
    }
}

struct AnimationState {
    enabled: bool,
    time: u32,
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(256, 128));

    let state = BoundData::new(
        AnimationState {
            enabled: false,
            time: 0,
        },
        |_| {},
    );

    let mut gui = Window::new(
        EgCanvas::new(display),
        Column::new()
            .add(
                DefaultTheme::primary_button("Animate")
                    .bind(&state)
                    .on_clicked(|state| state.enabled = !state.enabled),
            )
            .add(FillParent::both(Border::new(
                Canvas::with_properties(CanvasStyle::<Rgb888>::new(EgSize::new(256, 128)))
                    .with_input_handler(|_ctxt, input| {
                        state.update(|data| match input {
                            InputEvent::PointerEvent(_, event) => match event {
                                PointerEvent::Up => data.enabled = true,
                                PointerEvent::Down => data.enabled = false,
                                PointerEvent::Hover | PointerEvent::Drag => {}
                            },
                            InputEvent::KeyEvent(KeyEvent::KeyDown(Key::Space, _, _)) => {
                                data.enabled = !data.enabled;
                            }
                            _ => (),
                        });
                        true
                    })
                    .bind(&state)
                    .on_data_changed(|widget, state| {
                        let clear_color = widget.canvas_properties.clear_color;
                        let canvas = widget.canvas();

                        canvas.clear(clear_color).unwrap();

                        let t = state.time % 100;

                        let increment = if t < 50 { t } else { 50 - (t - 50) };

                        let rectangle = Rectangle::with_center(
                            canvas.center(),
                            EgSize {
                                width: 50 + increment,
                                height: 50 + increment,
                            },
                        )
                        .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1));
                        rectangle.draw(canvas).unwrap();

                        let circle = Circle::with_center(canvas.center(), 40 + increment)
                            .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1));
                        circle.draw(canvas).unwrap();
                    }),
            ))),
    );

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = SimWindow::new("GUI demonstration", &output_settings);

    loop {
        gui.canvas.target.clear(Rgb888::BACKGROUND_COLOR).unwrap();

        gui.update();
        gui.measure();
        gui.arrange();
        gui.draw().unwrap();

        state.update(|state| {
            if state.enabled {
                state.time = state.time.wrapping_add(1);
            }
        });

        // Update the window.
        window.update(&gui.canvas.target);

        // Handle key and mouse events.
        for event in window.events() {
            match convert_input(event) {
                Ok(input) => {
                    gui.input_event(input);
                }
                Err(true) => return,
                _ => {}
            }
        }

        // Wait for a little while.
        thread::sleep(Duration::from_millis(10));
    }
}
