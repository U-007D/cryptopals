#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate fluent_validator;

mod consts;
use consts::*;
mod byte_buffer;
use byte_buffer::ByteBuffer;
mod hex_byte_string;
use hex_byte_string::HexByteString;

#[cfg(test)]
mod unit_tests;

use fluent_validator::{FluentValidator, Validator};
