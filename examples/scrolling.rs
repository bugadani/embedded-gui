use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
    widgets::label::ascii::LabelConstructor,
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
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, PointerEvent, ScrollEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, row::Row, Cell},
        primitives::border::Border,
        scroll::Scroll,
    },
    Window,
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

struct ScrollData {
    current: i32,
    max: i32,
    reset: bool,
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(128, 64));

    let scroll_data = BoundData::new(
        ScrollData {
            current: 0,
            max: 0,
            reset: false,
        },
        |_| (),
    );
    // TODO: this example should also demonstrate a scrollbar and horizontal scroll widget
    let mut gui = Window::new(
        EgCanvas::new(display),
        Row::new(
            Cell::new(
                Column::new(
                    Cell::new(
                        Row::new(Cell::new(
                            Label::new("Scroll down")
                                .bind(&scroll_data)
                                .on_data_changed(|label, data| {
                                    label.text = if data.current == data.max {
                                        "Scroll back"
                                    } else if data.current == 0 {
                                        "Scroll down"
                                    } else {
                                        "Scroll more"
                                    };
                                }),
                        ))
                        .add(Cell::new(
                            DefaultTheme::primary_button("Reset")
                                .bind(&scroll_data)
                                .on_clicked(|data| data.reset = true),
                        )),
                    )
                    .weight(1),
                )
                .add(
                    Cell::new(Border::new(
                        Scroll::vertical(
                            Column::new(Cell::new(Label::new("S")))
                                .add(Cell::new(Label::new("c")))
                                .add(Cell::new(Label::new("r")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(
                                    DefaultTheme::primary_button("l")
                                        .bind(&scroll_data)
                                        .on_clicked(|data| {
                                            println!("Clicked at scroll offset: {}", data.current)
                                        }),
                                ))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("l")))
                                .add(Cell::new(Label::new("o")))
                                .add(Cell::new(Label::new("Scrollolo :)"))),
                        )
                        .friction(1)
                        .friction_divisor(2)
                        .bind(&scroll_data) // FIXME (maybe) - needs to be bound otherwise callback doesn't fire
                        .on_scroll_changed(|data, pos| {
                            data.current = pos.offset;
                            data.max = pos.maximum_offset;
                            data.reset = false;
                        })
                        .on_data_changed(|scroll, data| {
                            if data.reset {
                                scroll.scroll_to(0);
                            }
                        }),
                    ))
                    .weight(5),
                ),
            )
            .weight(3),
        )
        .add(Cell::new(DefaultTheme::vertical_scrollbar()).weight(1)),
    );

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = SimWindow::new("GUI demonstration", &output_settings);

    // In this example, the size of the widgets can't change so it's enough to measure once.
    gui.measure();

    loop {
        gui.canvas
            .target
            .clear(BinaryColor::BACKGROUND_COLOR)
            .unwrap();

        gui.update();
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
