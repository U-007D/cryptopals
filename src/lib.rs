#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]
#![feature(try_from)]

extern crate fluent_validator;

mod consts;
mod byte_buffer;
mod hex_char;

#[cfg(test)]
mod unit_tests;

pub fn run(_args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    Ok(())
}
