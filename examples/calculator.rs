use std::{fmt::Write, thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
    widgets::label::{ascii::LabelConstructor, MonoFontLabelStyling},
    EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget, mono_font::ascii::FONT_10X20, pixelcolor::BinaryColor,
    prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, PointerEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, row::Row},
        primitives::{
            fill::{FillParent, Right},
            spacing::Spacing,
        },
    },
    Window,
};
use heapless::{consts::U11, String};

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

#[derive(Copy, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Op {
    fn calc(self, current: i32, previous: i32) -> i32 {
        match self {
            Op::Add => previous.saturating_add(current),
            Op::Subtract => previous.saturating_sub(current),
            Op::Multiply => previous.saturating_mul(current),
            Op::Divide => {
                // I'm too lazy to add error handling.
                let div = if current == 0 { 1 } else { current };
                previous / div
            }
        }
    }

    fn bind(self, n: i32) -> PrevOp {
        match self {
            Op::Add => PrevOp::Add(n),
            Op::Subtract => PrevOp::Subtract(n),
            Op::Multiply => PrevOp::Multiply(n),
            Op::Divide => PrevOp::Divide(n),
        }
    }
}

#[derive(Copy, Clone)]
pub enum PrevOp {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
}

impl PrevOp {
    fn apply(self, n: i32) -> i32 {
        match self {
            PrevOp::Add(prev) => Op::Add.calc(prev, n),
            PrevOp::Subtract(prev) => Op::Subtract.calc(prev, n),
            PrevOp::Multiply(prev) => Op::Multiply.calc(prev, n),
            PrevOp::Divide(prev) => Op::Divide.calc(prev, n),
        }
    }
}

pub struct Calculator {
    pub previous: i32,
    pub current: i32,
    current_op: Option<Op>,
    prev_op: Option<PrevOp>,
    next_digit_clears: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            previous: 0,
            current: 0,
            current_op: None,
            prev_op: None,
            next_digit_clears: false,
        }
    }

    pub fn clear(&mut self) {
        self.previous = 0;
        self.current = 0;
        self.current_op = None;
        self.prev_op = None;
        self.next_digit_clears = false;
    }

    pub fn add_digit(&mut self, d: i32) {
        if self.next_digit_clears {
            self.next_digit_clears = false;
            self.current = 0;
        }
        if let Some(c) = self.current.checked_mul(10).and_then(|c| c.checked_add(d)) {
            self.current = c;
        }
    }

    pub fn delete_digit(&mut self) {
        self.current = self.current / 10;
    }

    pub fn set_op(&mut self, op: Op) {
        if self.current_op.is_some() {
            self.calc();
        }
        self.previous = self.current;
        self.current_op = Some(op);
        self.next_digit_clears = true;
    }

    pub fn update(&mut self) {
        if self.current_op.is_some() {
            self.calc();
        } else if let Some(prev_op) = self.prev_op {
            self.current = prev_op.apply(self.current);
        }
        self.current_op = None;
        self.next_digit_clears = true;
    }

    fn calc(&mut self) {
        match self.current_op {
            Some(op) => {
                self.prev_op = Some(op.bind(self.current));
                let prev = std::mem::replace(&mut self.previous, self.current);
                self.current = op.calc(self.current, prev);
            }
            None => {}
        }
    }
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(96, 96));

    let calculator = BoundData::new(Calculator::new(), |_data| {});

    let mut gui = Window::new(
        EgCanvas::new(display),
        Column::new()
            .spacing(1)
            .add(
                Spacing::new(
                    FillParent::horizontal(
                        Label::new(String::<U11>::from("0"))
                            .font(&FONT_10X20)
                            .bind(&calculator)
                            .on_data_changed(|label, calc| {
                                label.text.clear();
                                write!(label.text, "{}", calc.current).unwrap();
                            }),
                    )
                    .align_horizontal(Right),
                )
                .all(4),
            )
            .add(
                Row::new()
                    .spacing(1)
                    .add(
                        DefaultTheme::primary_button_stretched("CE")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.clear()),
                    )
                    .weight(2)
                    .add(
                        DefaultTheme::secondary_button_stretched("<")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.delete_digit()),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::primary_button_stretched("/")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.set_op(Op::Divide)),
                    )
                    .weight(1),
            )
            .weight(1)
            .add(
                Row::new()
                    .spacing(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("7")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(7)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("8")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(8)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("9")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(9)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::primary_button_stretched("x")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.set_op(Op::Multiply)),
                    )
                    .weight(1),
            )
            .weight(1)
            .add(
                Row::new()
                    .spacing(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("4")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(4)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("5")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(5)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("6")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(6)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::primary_button_stretched("-")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.set_op(Op::Subtract)),
                    )
                    .weight(1),
            )
            .weight(1)
            .add(
                Row::new()
                    .spacing(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("1")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(1)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("2")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(2)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("3")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(3)),
                    )
                    .weight(1)
                    .add(
                        DefaultTheme::primary_button_stretched("+")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.set_op(Op::Add)),
                    )
                    .weight(1),
            )
            .weight(1)
            .add(
                Row::new()
                    .spacing(1)
                    .add(
                        DefaultTheme::secondary_button_stretched("0")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.add_digit(0)),
                    )
                    .weight(3)
                    .add(
                        DefaultTheme::primary_button_stretched("=")
                            .bind(&calculator)
                            .on_clicked(|calculator| calculator.update()),
                    )
                    .weight(1),
            )
            .weight(1),
    );

    println!("Size of struct: {}", std::mem::size_of_val(&gui.root));
    fn print_type_of<T>(_: &T) {
        println!("type of tree: {}", std::any::type_name::<T>());
        println!("length of type: {}", std::any::type_name::<T>().len());
    }

    print_type_of(&gui.root);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = SimWindow::new("GUI demonstration", &output_settings);

    loop {
        gui.canvas
            .target
            .clear(BinaryColor::BACKGROUND_COLOR)
            .unwrap();

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
