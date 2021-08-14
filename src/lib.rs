//! embedded-gui
//! ============
//!
//! `embedded-gui` is an experimental `no_std`, `no_alloc`, cross-platform, composable Rust GUI
//! toolkit.
//!
//! `embedded-gui` consists of two parts: the main crate, and a platform-specific backend.
//! The main crate contains layout containers, composable base widgets, and the event handling
//! framework.
//! Backend crates define how each widget is rendered, and they may also contain custom widgets or
//! backend-specific extensions to the base widgets.
//!
//! Widgets
//! -------
//!
//! In an `embedded-gui` GUI, the most basic unit is called a widget. Widgets define behavior and/or
//! appearance. A GUI is defined as a tree of widgets. This means that widgets may either be leaf
//! widgets, wrapper widgets or layout containers. Wrapper widgets and layout containers contain
//! other widget(s).
//!
//! The set of widgets provided by `embedded-gui` are listed in the [`widgets`] module.
//!
//! Data binding
//! ------------
//!
//! `embedded-gui` widgets support a limited form of data binding. Data binding can be used to
//! connect widgets together or to pass data to the outside of the GUI.
//!
//! Widget states
//! -------------
//!
//! Some widgets define "states", which are used to communicate changes to the descendants of the
//! widget. For example, states can be used to change the border color of a button.
//!
//! States always propagate from the outside in, from a wrapper widget to the wrapped one.
//! Usually, when a widget defines it's own state, it will not propagate the parent's state further.
//!
//! Supported platforms
//! -------------------
//!
//!  * [`embedded-graphics`]: [`backend-embedded-graphics`]
//!
//! [`embedded-graphics`]: https://github.com/embedded-graphics/embedded-graphics
//! [`backend-embedded-graphics`]: https://github.com/bugadani/embedded-gui/backend-embedded-graphics

#![no_std]

pub mod data;
pub mod geometry;
pub mod input;
pub mod state;
pub mod widgets;

use crate::{
    geometry::{measurement::MeasureSpec, MeasuredSize, Position},
    input::{
        controller::{DefaultInputController, InputController},
        event::InputEvent,
    },
    widgets::Widget,
};

pub mod prelude {
    pub use crate::{
        data::WidgetData,
        widgets::{utils::wrapper::WrapperBindable, Widget},
        WidgetRenderer, Window,
    };
}

pub trait WidgetRenderer<C: Canvas> {
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error>;
}

pub trait Canvas {
    type Error;

    fn size(&self) -> MeasuredSize;
}

pub struct Window<C, W, I>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
    I: InputController,
{
    pub canvas: C,
    pub root: W,
    pub input_controller: I,
}

impl<C, W> Window<C, W, DefaultInputController>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub fn new(canvas: C, mut root: W) -> Self {
        root.attach(0, 0);
        Window {
            canvas,
            root,
            input_controller: DefaultInputController::new(),
        }
    }
}

impl<C, W, I> Window<C, W, I>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
    I: InputController,
{
    pub fn with_input_controller<I2>(self, input_controller: I2) -> Window<C, W, I2>
    where
        I2: InputController,
    {
        Window {
            canvas: self.canvas,
            root: self.root,
            input_controller,
        }
    }

    pub fn update(&mut self) {
        self.root.update();
    }

    pub fn measure(&mut self) {
        self.root
            .measure(MeasureSpec::from_measured_at_most(self.canvas.size()));
    }

    pub fn arrange(&mut self) {
        self.root.arrange(Position { x: 0, y: 0 });
    }

    pub fn draw(&mut self) -> Result<(), C::Error> {
        self.root.draw(&mut self.canvas)
    }

    pub fn input_event(&mut self, event: InputEvent) {
        self.input_controller.input_event(&mut self.root, event);
    }
}
