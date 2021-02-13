/// This module provides the core language library
/// And compiler-magic FFI bindings.

pub mod ffi;
pub mod extract;

pub mod math;
pub mod io;
pub mod control;
pub mod logic;

use ffi::{FFI, FFIFunction};

// Returns the core FFI used by Passerine.
// Implements basic langauge features, like addition.
pub fn ffi_core() -> FFI {
    let mut ffi = FFI::new();

    // math
    // all these x are redundant :\
    ffi.add("add", FFIFunction::new(Box::new(math::add))).unwrap();
    ffi.add("sub", FFIFunction::new(Box::new(math::sub))).unwrap();
    ffi.add("mul", FFIFunction::new(Box::new(math::mul))).unwrap();
    ffi.add("div", FFIFunction::new(Box::new(math::div))).unwrap();

    // io
    ffi.add("println", FFIFunction::new(Box::new(io::println))).unwrap();
    ffi.add("print", FFIFunction::new(Box::new(io::print))).unwrap();

    // control
    ffi.add("if", FFIFunction::new(Box::new(control::if_choice))).unwrap();

    // logic
    ffi.add("equal", FFIFunction::new(Box::new(logic::equal))).unwrap();

    return ffi;
}
