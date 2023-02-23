use std::ffi::*;

use crate::raw::*;

#[derive(Clone, Copy)]
#[allow(dead_code, unused_variables)]
enum MarketDataType {
    Last,
    Ask,
    Bid,
}

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
        _ => panic!("Ask() returned an unexpected value"),
    }

}

fn cash_value(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: c_double = unsafe { CashValue(account.as_ptr()) };
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

fn connected(show_message: bool) -> bool {
    let show_message: c_int = c_int::try_from(match show_message {
        true => 1,
        false => 0,
    }).unwrap();
    let result: c_int = unsafe { Connected(show_message) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Connected() returned an unexpected value"),
    }
}

fn market_data(instrument: &str, market_data_type: MarketDataType) -> f64 {
    let instrument: CString = CString::new(instrument).unwrap();
    let data_type: c_int = c_int::try_from(match market_data_type {
        Last => 0,
        Bid => 1,
        Ask => 2,
    }).unwrap();
    let result: c_double = unsafe { MarketData(instrument.as_ptr(), data_type) };
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

fn subscribe_market_data(instrument: &str) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let result: c_int = unsafe { SubscribeMarketData(instrument.as_ptr()) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask returned an unexpected value"),
    }
}

// needs a way to parse the timestamp as a string for the ffi (yyyyMMddHHmmss)
// fn ask_playback(instrument: &str, price: f64, size: i32, timestamp:)

