// use cxx::*;
use std::ffi::*;

use crate::raw::*;

// need unwrap errors handled
// bool returns need to be success/fail errors?
// would be good practice error handling, and also make interface more rusty
// or save that for my own program?

fn ask(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: c_int = unsafe { Ask(instrument.as_ptr(), price, size) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask returned an unexpected value"),
    }

}

// needs a way to parse the timestamp as a string for the ffi (yyyyMMddHHmmss)
// fn ask_playback(instrument: &str, price: f64, size: i32, timestamp:)

