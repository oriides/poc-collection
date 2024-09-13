// include the bindings to our C library from the bindings.rs file
mod bindings;
// include the wrapper module (wrapper.rs). It contains a wrapper struct for the C struct
mod wrapper;

// This is basically the Rust version of a "using namespace" declaration
// it enables the use of e.g. MyStructWrapper directly without specifying the module where it lives.
use crate::bindings::*;
use crate::wrapper::*;

/// The main function of our program
fn main() {
    // It is possible to interact with the CStruct directly using the bindings.
    // ⚠ However: This requires the use of unsafe every time you interact with the struct
    // SAFETY: There aren't any guarantees…
    unsafe {
        let s: *mut MyCStruct = new(2);
        println!("my_struct->x = {}", (*s).x);
        multiply(&mut *s, 3);
        println!("my_struct->x = {}", (*s).x);
    }

    // Instead you should create a safe "API" around the struct by building a wrapper struct.
    // I have done exactly this in the `wrapper.rs` file which has been included in this file.
    // In order to keep this example brief, I'm skipping most of the else statements after my "if let Ok/Some" statements, where you would normally do the error handling. As a result if there is an error somewhere the program would just not do anything.
    // Try to create a CStruct through the MyStructWrapper and run the following code if successful
    if let Ok(my_struct) = MyStructWrapper::new(2) {
        // Try to get x of our Struct, if successful: print x
        if let Some(x) = my_struct.get_x() {
            println!("my_struct->x = {}", x);
        }

        // multiply the x value of MyStruct by 3
        my_struct.multiply(3);

        // Try to get x of our Struct, if successful: print x
        if let Some(x) = my_struct.get_x() {
            println!("my_struct->x = {}", x);
        }
    } // my_struct falls out of scope → drop() (of MyStructWrapper) is called implicitly → drop(…) (of MyCStruct) is called → there is no memory leak
}
