#![feature(core_intrinsics, heap_api, raw, slice_bytes)]
#[macro_use]
extern crate log;

pub mod ast;
pub mod gc;
pub mod interner;