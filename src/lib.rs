#![feature(layout_for_ptr)]
#![allow(unused_imports)]

//! Library for copying certain memory regions and storing their pointers inside a hash map collection.
//!
//! **MemMap** offers also a simple way of [IPC](https://en.wikipedia.org/wiki/Inter-process_communication). <br>
//! All processes implementing this library can access each other's maps without any blocking or security.
//!
//! ## Usage
//! This lib can be used as Rust crate or system library
//!
//! ```toml
//! [dependencies.mem-map]
//! path = "https://github.com/VulcoLibs/mem-map"
//! ```
//!
//! To build a system library, simply go to mem-map's Cargo.toml file and follow the instructions.
//!
//! ## Inner Functions
//! **Inner Functions** are functions that work at own process' map.
//!
//! - [init](init)
//! - [insert_cp](insert_cp)
//! - [insert_mv](insert_mv)
//! - [get](get)
//! - ~~[get_from_reg](get_from_reg)~~
//! - [remove](remove)
//!
//! ## Extern Functions
//! **Extern Functions** are functions that work at specified process' map.
//!
//! - [get_extern](get_extern)
//! - [insert_extern](insert_extern)
//! - [id_exists](id_exists)


#[doc(hidden)]
#[macro_use]
extern crate lazy_static;


#[doc(hidden)] mod bindings;
#[doc(hidden)] mod private;
pub mod errors;
mod mem_map;
mod exports;

use bindings::*;
use private::*;
use errors::*;
use mem_map::*;
pub use exports::*;


/// Registry path to the **MemMap** keys. <br>
/// Full path: `HKEY_CURRENT_USER\SOFTWARE\VulcoLibs\MemMap`
const REG_PATH: &'static str = r"Software\VulcoLibs\MemMap";

#[doc(hidden)] const NULL: *mut c_void = std::ptr::null_mut();
#[doc(hidden)] const HKCU: RegKey = RegKey::predef(HKEY_CURRENT_USER);
