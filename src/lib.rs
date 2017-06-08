//! # mem
//! A rather generic memory (block) interface.
//! Use Mutexes if you need thread safety, this doesn't provide any.
//!
//! This crate is mainly for building emulators and such.
//!
//! ## But.. why?!
//! Because abstraction is great.
//! Well, this is really just an abstraction around byte addressable storage.
//! Doesn't have to be RAM, could be a file or even a block device.
//! Or some sort of remote memory interface. Or a live memory debugger/editor for an emulator!
//! Let your imagination go wild! :)

// Errors
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

// Modules
// Since this crate is split over a couple of files, this is needed.
pub mod errors;
mod interface;
mod helpers;

pub mod std_impls;

// Export the sub modules globally at crate level.
pub use interface::*;
pub use helpers::*;
