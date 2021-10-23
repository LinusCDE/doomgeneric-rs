//! This lib exposes the basic set of functions of the doomgeneric port to rust.
//!
//! This is currently not really polished and I might change a lot in future versions.
//!
//! For the time being, this lib will be not threadsafe and not support async.
//! If you need this in a multithreaded environment, it's probably best to start this
//! in a seperate thread and communicate using channels.

pub mod game;
pub mod input;
