//! The implementation for `Device` is very large so it's been split into a bunch of modules.

mod commands;
mod entities;
mod file_transfer;
mod from;
pub mod public;
mod receive;
mod screen_capture;
mod send;
mod trivial;

type CommandId = u8;
