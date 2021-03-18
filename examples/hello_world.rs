use std::{thread, time::Duration};

use backend_embedded_graphics::EgCanvas;
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::{BoundData, WidgetData},
    input::InputEvent,
    widgets::{
        border::Border,
        button::Button,
        fill::{Bottom, Center, FillParent},
        label::{Label, LabelConstructor},
        spacing::Spacing,
        DataHolder,
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
                Ok(InputEvent::PointerUp(Position {
                    x: point.x,
                    y: point.y,
                }))
            }
            SimulatorEvent::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                point,
            } => {
                MOUSE_DOWN = true;
                Ok(InputEvent::PointerDown(Position {
                    x: point.x,
                    y: point.y,
                }))
            }
            SimulatorEvent::MouseMove { point } => Ok(if MOUSE_DOWN {
                InputEvent::PointerMove(Position {
                    x: point.x,
                    y: point.y,
                })
            } else {
                InputEvent::PointerHover(Position {
                    x: point.x,
                    y: point.y,
                })
            }),
            SimulatorEvent::Quit => Err(true),
            _ => Err(false),
        }
    }
}

fn main() {
    let display = SimulatorDisplay::<BinaryColor>::new(EgSize::new(64, 32));

    let flag = BoundData::new(true, |data| {
        println!("Data changed to {:?}", data);
    });

    let mut gui = Window::new(
        EgCanvas::new(display),
        Spacing::new(
            Button::new(
                Border::new(
                    FillParent::both(Label::new("Click me").bind(&flag).on_data_changed(
                        |mut widget, data| {
                            widget.text = if *data.get() { "on" } else { "off" };
                        },
                    ))
                    .align_horizontal(Center)
                    .align_vertical(Bottom),
                )
                .border_color(BinaryColor::Off)
                .on_state_changed(|widget, state| match state.state() {
                    Button::STATE_HOVERED => widget.border_color(BinaryColor::On),
                    Button::STATE_PRESSED => widget.border_color(BinaryColor::Off),
                    _ => widget.border_color(BinaryColor::Off),
                }),
            )
            .bind(&flag)
            .on_clicked(|data| {
                data.update(|mut data| *data = !*data);
            }),
        )
        .all(4),
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
