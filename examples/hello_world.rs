use std::{thread, time::Duration};

use backend_embedded_graphics::{
    widgets::primitives::{background::BackgroundStyle, border::BorderStyle},
    EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::{BoundData, WidgetData},
    input::event::{InputEvent, PointerEvent},
    widgets::{
        button::Button,
        label::{Label, LabelConstructor},
        primitives::{
            background::Background,
            border::Border,
            fill::{Bottom, Center, FillParent},
            spacing::Spacing,
        },
        DataHolder, Widget,
    },
    Position, WidgetState, Window,
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

fn update_button_background<W: Widget>(
    widget: &mut Background<W, BackgroundStyle<BinaryColor>>,
    state: WidgetState,
) {
    match state.state() {
        Button::STATE_HOVERED => widget.background_color(BinaryColor::Off),
        Button::STATE_PRESSED => widget.background_color(BinaryColor::On),
        _ => widget.background_color(BinaryColor::Off),
    }
}

fn update_button_border<W: Widget>(
    widget: &mut Border<W, BorderStyle<BinaryColor>>,
    state: WidgetState,
) {
    match state.state() {
        Button::STATE_HOVERED => widget.border_color(BinaryColor::On),
        Button::STATE_PRESSED => widget.border_color(BinaryColor::Off),
        _ => widget.border_color(BinaryColor::Off),
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
                Background::new(
                    Border::new(
                        FillParent::both(Label::new("Click me").bind(&flag).on_data_changed(
                            |widget, data| {
                                widget.text = if *data.get() { "on" } else { "off" };
                            },
                        ))
                        .align_horizontal(Center)
                        .align_vertical(Bottom),
                    )
                    .border_color(BinaryColor::Off)
                    .on_state_changed(update_button_border),
                )
                .background_color(BinaryColor::Off)
                .on_state_changed(update_button_background),
            )
            .bind(&flag)
            .on_clicked(|data| {
                data.update(|mut data| *data = !*data);
                println!("Clicked!");
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
