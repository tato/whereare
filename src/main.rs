#![deny(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::Error;

/// Window creation, event handling
mod app;
/// Global state
mod world;
/// Generate maps
mod map;

fn main() -> Result<(), Error> {
    app::run()
}