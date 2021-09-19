#![deny(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::Error;

/// Window creation, event handling
mod app;
/// IMGUI
mod gui;
/// Generate maps
mod map;
/// Global state
mod world;

fn main() -> Result<(), Error> {
    app::run()
}
