#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "nrf51"]
#![crate_type = "rlib"]
#![warn(missing_docs)]

extern crate cortexm0;
extern crate nrf5x;

#[allow(unused_imports)]
#[macro_use(debug, debug_verbose, debug_gpio)]
extern crate kernel;

pub mod clock;
pub mod chip;
pub mod crt1;
pub mod uart;
pub mod radio;

pub use crt1::init;
