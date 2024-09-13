// since the standard C int type is is not the same as a standard Rust int type we need to include the C int type for our Rust program
use std::os::raw::c_int;

/// A Rust-idiomatic representation of the struct from our C library  
/// The "repr(C)" is an instruction for the Rust compiler to represent the data of the struct in a C code compliant way
#[repr(C)]
pub struct MyCStruct {
    pub x: c_int,
}

// Declaration of the external C functions = bindings.  
// The "C" tells the compiler to adhere to the C ABI on function calls
extern "C" {
    pub fn new(i: i32) -> *mut MyCStruct;
    pub fn drop(s: *mut MyCStruct);
    pub fn multiply(s: *mut MyCStruct, factor: c_int);
}
