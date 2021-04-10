use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::default::primary_button, widgets::label::ascii::LabelConstructor, EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    input::event::{InputEvent, PointerEvent, ScrollEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, Cell},
        primitives::border::Border,
        scroll::Scroll,
    },
    Position, Window,
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

            SimulatorEvent::MouseWheel { scroll_delta, .. } => {
                // TODO: We could break this down into multiple scroll events
                Ok(InputEvent::ScrollEvent(if scroll_delta.y != 0 {
                    ScrollEvent::VerticalScroll(scroll_delta.y * 4)
                } else {
                    ScrollEvent::HorizontalScroll(scroll_delta.x * 4)
                }))
            }

            SimulatorEvent::Quit => Err(true),
            _ => Err(false),
        }
    }
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(32, 32));

    let dummy_data = BoundData::new((), |_| ());
    // TODO: this example should also demonstrate a scrollbar and horizontal scroll widget
    let mut gui = Window::new(
        EgCanvas::new(display),
        Border::new(
            Scroll::vertical(
                Column::new(Cell::new(Label::new("S")))
                    .add(Cell::new(Label::new("c")))
                    .add(Cell::new(Label::new("r")))
                    .add(Cell::new(Label::new("o")))
                    .add(Cell::new(
                        primary_button("l")
                            .bind(&dummy_data)
                            .on_clicked(|_| println!("Clicked")),
                    ))
                    .add(Cell::new(Label::new("l")))
                    .add(Cell::new(Label::new("o")))
                    .add(Cell::new(Label::new("l")))
                    .add(Cell::new(Label::new("o"))),
            )
            .bind(&dummy_data) // FIXME (maybe) - needs to be bound otherwise callback doesn't fire
            .on_scroll_changed(|_, _pd| {
                //println!("Scroll offset: {:?}", pd);
            }),
        ),
    );

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = SimWindow::new("GUI demonstration", &output_settings);

    loop {
        gui.canvas.target.clear(BinaryColor::Off).unwrap();

        gui.update();
        gui.measure();
        gui.arrange();
        gui.draw().unwrap();

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
