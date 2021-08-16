embedded-gui
============

`embedded-gui` is an experimental `no_std`, `no_alloc`, cross-platform, composable Rust GUI toolkit.

`embedded-gui` consists of two parts: the main crate, and a platform-specific backend.
The main crate contains layout containers, composable base widgets, and the event handling framework.
Backend crates define how each widget is rendered, and they may also contain custom widgets or
backend-specific extensions to the base widgets.

Supported platforms
-------------------

 * `embedded-graphics`: [platform][embedded-graphics] - [backend][backend-embedded-graphics]

[embedded-graphics]: https://github.com/embedded-graphics/embedded-graphics
[backend-embedded-graphics]: https://github.com/bugadani/embedded-gui/backend-embedded-graphics

Development setup
-----------------

### Minimum supported Rust version

The minimum supported Rust version for embedded-text is 1.51.0 or greater. Ensure you have the latest stable version of Rust installed, preferably through https://rustup.rs.
