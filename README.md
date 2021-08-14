embedded-gui
============

`embedded-gui` is an experimental `no_std`, `no_alloc`, cross-platform, composable Rust GUI toolkit.

`embedded-gui` consists of two parts: the main crate, and a platform-specific backend.
The main crate contains layout containers, composable base widgets, and the event handling framework.
Backend crates define how each widget is rendered, and they may also contain custom widgets or
backend-specific extensions to the base widgets.

Supported platforms
-------------------

 * [`embedded-graphics`]: [`backend-embedded-graphics`]

[`embedded-graphics`]: https://github.com/embedded-graphics/embedded-graphics
[`backend-embedded-graphics`]: https://github.com/bugadani/embedded-gui/backend-embedded-graphics
