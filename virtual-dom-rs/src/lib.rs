//!

#![feature(use_extern_macros)]

extern crate wasm_bindgen;

// Used so that `html!` calls work when people depend on this crate since `html!` needs
// access to `Closure` when creating event handlers.
pub use wasm_bindgen::prelude::Closure;

extern crate web_sys;

#[macro_use]
pub mod html_macro;
pub use html_macro::*;

pub mod virtual_node;
pub use virtual_node::*;

mod diff;
pub use diff::*;

mod patch;
pub use patch::*;
