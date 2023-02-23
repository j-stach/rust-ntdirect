use std::ffi::*;

use crate::raw::*;

#[derive(Clone, Copy)]
#[allow(dead_code, unused_variables)]
enum MarketDataType {
    Last,
    Ask,
    Bid,
}

fn parse_datetime() {}

fn separate(list: String) -> Vec<String> {
    let separator: char = '|';
    // Holds index values of separator characters
    let mut separators = list.char_indices().filter(|(_, char)| char == &separator).map(|(index, _)| index);
    let separator_count: usize = separators.clone().count();
    let mut separated: Vec<String> = Vec::with_capacity(separator_count + 1);

    let mut start: usize = 0;
    while let Some(index) = separators.next() {
        let chunk: &str = &list[start..index];
        if !chunk.is_empty() {
            separated.push(chunk.to_string())
        }
        start = index + 1;
    }
    
    let last_chunk: &str = &list[start..];
    if !last_chunk.is_empty() {
        separated.push(last_chunk.to_string())
    }

    return separated
}

// other custom types? OrderId etc that implement formatting

// need unwrap errors handled
// bool returns need to be success/fail errors?
// would be good practice error handling, and also make interface more rusty
// or save that for my own program?

fn ask(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: i32 = unsafe { Ask(instrument.as_ptr(), price, size) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }

}

// needs a way to parse the timestamp as a string for the ffi (yyyyMMddHHmmss)
// fn ask_playback(instrument: &str, price: f64, size: i32, timestamp:)

fn avg_entry_price(instrument: &str, account: &str) -> f64 {
    let instrument: CString = CString::new(instrument).unwrap();
    let account: CString = CString::new(account).unwrap();
    let result: f64 = unsafe { AvgEntryPrice(instrument.as_ptr(), account.as_ptr())};
    return result
}

fn avg_fill_price(order_id: &str) -> f64 {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: f64 = unsafe { AvgFillPrice(order_id.as_ptr())};
    return result
}

fn bid(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: i32 = unsafe { Bid(instrument.as_ptr(), price, size) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }
}

// needs a way to parse the timestamp as a string for the ffi (yyyyMMddHHmmss)
// fn ask_playback(instrument: &str, price: f64, size: i32, timestamp:)

fn buying_power(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: f64 = unsafe { BuyingPower(account.as_ptr()) };
    return result   
}

fn cash_value(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: f64 = unsafe { CashValue(account.as_ptr()) };
    return result
}

// needs order type & tif enums? look into valid commands before building this, may be able to make it easier
// fn command(command: &str, account: &str, instrument: &str, action: &str, quantity: i32, order_type: &str, limit_price: f64, stop_price: f64, tif: &str, oco: &str, order_id: &str, strategy: &str, strategy_id: &str) -> bool {
//     
// }

fn confirm_orders(confirm: bool) -> bool {
    let confirm: c_int = match confirm {
        true => 1,
        false => 0,
    };
    let result: i32 = unsafe { ConfirmOrders(confirm) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("ConfirmOrders() returned an unexpected value"),
    }
}

fn connected(show_message: bool) -> bool {
    let show_message: c_int = c_int::try_from(match show_message {
        true => 1,
        false => 0,
    }).unwrap();
    let result: i32 = unsafe { Connected(show_message) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Connected() returned an unexpected value"),
    }
}

fn filled(order_id: &str) -> i32 {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: i32 = unsafe { Filled(order_id.as_ptr())};
    return result
}

fn last(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: i32 = unsafe { Last(instrument.as_ptr(), price, size) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Last() returned an unexpected value"),
    }
}

// needs a way to parse the timestamp as a string for the ffi (yyyyMMddHHmmss)
// fn last_playback(instrument: &str, price: f64, size: i32, timestamp:)

fn market_data(instrument: &str, market_data_type: MarketDataType) -> f64 {
    let instrument: CString = CString::new(instrument).unwrap();
    let data_type: c_int = c_int::try_from(match market_data_type {
        Last => 0,
        Bid => 1,
        Ask => 2,
    }).unwrap();
    let result: f64 = unsafe { MarketData(instrument.as_ptr(), data_type) };
    return result
}

// enum for position?
fn market_position(instrument: &str, account: &str) -> i32 {
    let instrument: CString = CString::new(instrument).unwrap();
    let account: CString = CString::new(account).unwrap();
    let result: c_int = unsafe { MarketPosition(instrument.as_ptr(), account.as_ptr())};
    let result: i32 = i32::try_from(result).unwrap();
    return result
}

// needs to convert from pointer to i8 bytes to rust string, may want custom order_id type
 fn new_order_id() -> String {
    let result: &CStr = unsafe { CStr::from_ptr(NewOrderId()) };
    let result: String = result.to_str().unwrap().to_string();
    return result
 }

fn orders(account: &str) -> Vec<String> {
    let account: CString = CString::new(account).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(Orders(account.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}

// may need custom types
fn order_status(order_id: &str) -> String {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(OrderStatus(order_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    return result
}

fn realized_pnl(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: f64 = unsafe { RealizedPnL(account.as_ptr())};
    return result
}

// check integer types for ports, unsigned?
fn setup(host: &str, port: i32) -> bool {
    let host: CString = CString::new(host).unwrap();
    let port: c_int = c_int::try_from(port).unwrap();

    let result: i32 = unsafe { SetUp(host.as_ptr(), port) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }
}

// need to convert to rust string and parse
fn get_stop_orders(strategy_id: &str) -> Vec<String> {
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(StopOrders(strategy_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}

// need to convert to rust string
fn strategies(account: &str) -> Vec<String> {
    let account: CString = CString::new(account).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(Strategies(account.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}
   
// enum like market position?
fn strategy_position(strategy_id: &str) -> i32 {
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let result: i32 = unsafe { StrategyPosition(strategy_id.as_ptr()) };
    return result
}

fn subscribe_market_data(instrument: &str) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let result: i32 = unsafe { SubscribeMarketData(instrument.as_ptr()) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}

// need to convert to rust string
fn get_target_orders(strategy_id: &str) -> Vec<String> {
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(Orders(strategy_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}

fn tear_down() -> bool {
    let result: i32 = unsafe { TearDown() };    
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}

fn unsubscribe_market_data(instrument: &str) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let result: i32 = unsafe { UnsubscribeMarketData(instrument.as_ptr()) };
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}
