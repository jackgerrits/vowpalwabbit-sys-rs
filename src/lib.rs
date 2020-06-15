#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Rust bindings for the [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit) C-binding surface.
//!
//! Experimental bindings using the new C binding layer.
//!

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::mem;

    #[test]
    fn test_initialize_and_parse_learn_example() {
        unsafe {
            let errString = VWCreateErrorString();
            let mut options = mem::MaybeUninit::uninit();
            let result = VWCreateOptions(options.as_mut_ptr(), errString);
            assert_eq!(result, VW_SUCCESS);

            let options = options.assume_init();
            let command_line_str = CString::new("--quiet").unwrap();
            let result = VWOptionsSetBool(options, command_line_str.as_ptr(), true, errString);
            assert_eq!(result, VW_SUCCESS);

            let result = VWDestroyOptions(options, errString);
            assert_eq!(result, VW_SUCCESS);
            VWDestroyErrorString(errString);
        }
    }
}
