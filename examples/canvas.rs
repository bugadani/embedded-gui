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
    sdl2::MouseButton, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, PointerEvent},
    prelude::*,
    widgets::{border::Border, fill::FillParent, layouts::linear::Column},
};

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
