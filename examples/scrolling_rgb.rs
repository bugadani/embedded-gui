use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::default::DefaultTheme, widgets::label::ascii::LabelConstructor, EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{RgbColor, Size as EgSize},
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window as SimWindow,
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

#[derive(Debug)]
enum ScrollOp {
    None,
    Reset,
    Scrollbar,
}

#[derive(Debug)]
struct ScrollData {
    current: i32,
    max: i32,
    viewport_size: i32,
    op: ScrollOp,
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(192, 160));

    let scroll_data = BoundData::new(
        ScrollData {
            current: 0,
            max: 0,
            viewport_size: 0,
            op: ScrollOp::None,
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
                                .on_clicked(|data| data.op = ScrollOp::Reset),
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
                            data.viewport_size = pos.viewport_size;
                            data.op = ScrollOp::None;
                        })
                        .on_data_changed(|scroll, data| match data.op {
                            ScrollOp::None => {}
                            ScrollOp::Reset => scroll.scroll_to(0),
                            ScrollOp::Scrollbar => scroll.set_position(data.current),
                        }),
                    ))
                    .weight(5),
                ),
            )
            .weight(1),
        )
        .add(Cell::new(
            DefaultTheme::vertical_scrollbar()
                .bind(&scroll_data)
                .on_data_changed(|scrollbar, data| {
                    // TODO: this might be a common use case to create a connector type
                    let scrollbar_height = scrollbar.bounds.size.height;
                    let scrollview_height = data.viewport_size as u32;
                    let scrollview_data_height = (data.max + data.viewport_size) as u32;

                    if scrollview_data_height > 0 {
                        scrollbar.properties.set_length(
                            (scrollbar_height * scrollview_height) / scrollview_data_height,
                        );
                        fn map(x: i32, x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
                            if x1 == x0 {
                                y0
                            } else {
                                ((y1 - y0) * (x - x0)) / (x1 - x0) + y0
                            }
                        }
                        scrollbar.set_range(0..=data.max);
                        scrollbar.set_value(map(
                            data.current,
                            0,
                            data.max,
                            0,
                            *scrollbar.limits.end(),
                        ));
                    }
                })
                .on_value_changed(|data, value| {
                    data.current = value;
                    data.op = ScrollOp::Scrollbar;
                }),
        )),
    );

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = SimWindow::new("ScrollWidget & scrollbar", &output_settings);

    // In this example, the size of the widgets can't change so it's enough to measure once.
    gui.measure();

    loop {
        gui.canvas.target.clear(Rgb888::BLACK).unwrap();

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
