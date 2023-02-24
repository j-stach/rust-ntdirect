use std::ffi::*;
use chrono::{DateTime, Utc};

use crate::raw::*;


// need to add error types for bool returns

// custom enums for market position, order type, tif, oco

// custom types/type aliases for order id, strategy id, etc?

// move these helper functions into a new file?
#[derive(Clone, Copy)]
#[allow(dead_code, unused_variables)]
enum MarketDataType {
    Last,
    Ask,
    Bid,
}

fn string_from_datetime(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y%m%d%H%M%S").to_string()
}

fn separate(list: String) -> Vec<String> {
    let separator: char = '|';

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

// Foreign functions begin here

fn ask(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();
    let result: c_int = unsafe { Ask(instrument.as_ptr(), price, size) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }

}

fn ask_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();
    let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

    let result: c_int = unsafe { AskPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("AskPlayback() returned an unexpected value"),
    }
}

fn avg_entry_price(instrument: &str, account: &str) -> f64 {
    let instrument: CString = CString::new(instrument).unwrap();
    let account: CString = CString::new(account).unwrap();
    let result: c_double = unsafe { AvgEntryPrice(instrument.as_ptr(), account.as_ptr())};
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

fn avg_fill_price(order_id: &str) -> f64 {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: c_double = unsafe { AvgFillPrice(order_id.as_ptr())};
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

fn bid(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: c_int = unsafe { Bid(instrument.as_ptr(), price, size) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }
}

fn bid_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

    let result: c_int = unsafe { BidPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("BidPlayback() returned an unexpected value"),
    }
}

fn buying_power(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: c_double = unsafe { BuyingPower(account.as_ptr()) };
    let result: f64 = f64::try_from(result).unwrap();
    return result   
}

fn cash_value(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: c_double = unsafe { CashValue(account.as_ptr()) };
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

// look into valid commands, may be able to make it easier
fn command(command: &str, account: &str, instrument: &str, action: &str, size: i32, order_type: &str, limit_price: f64, stop_price: f64, 
           tif: &str, oco: &str, order_id: &str, strategy: &str, strategy_id: &str) -> bool {

    let command: CString = CString::new(command).unwrap();
    let account: CString = CString::new(account).unwrap();
    let instrument: CString = CString::new(instrument).unwrap();
    let action: CString = CString::new(action).unwrap();
    let limit_price: c_double = c_double::try_from(limit_price).unwrap();
    let stop_price: c_double = c_double::try_from(stop_price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();
    let order_type: CString = CString::new(order_type).unwrap();
    let order_id: CString = CString::new(order_id).unwrap();
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let strategy: CString = CString::new(strategy).unwrap();
    let tif: CString = CString::new(tif).unwrap();
    let oco: CString = CString::new(oco).unwrap();

    let result: c_int = unsafe { Command(command.as_ptr(), account.as_ptr(), instrument.as_ptr(), 
                                  action.as_ptr(), size, order_type.as_ptr(), limit_price, stop_price, 
                                  tif.as_ptr(), oco.as_ptr(), strategy.as_ptr(), strategy_id.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("BidPlayback() returned an unexpected value"),
    }
}

fn confirm_orders(confirm: bool) -> bool {
    let confirm: c_int = match confirm {
        true => 1,
        false => 0,
    };
    let result: c_int = unsafe { ConfirmOrders(confirm) };
    let result: i32 = i32::try_from(result).unwrap();
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
    let result: c_int = unsafe { Connected(show_message) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Connected() returned an unexpected value"),
    }
}

fn filled(order_id: &str) -> i32 {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: c_int = unsafe { Filled(order_id.as_ptr())};
    let result: i32 = i32::try_from(result).unwrap();
    return result
}

fn last(instrument: &str, price: f64, size: i32) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();

    let result: c_int = unsafe { Last(instrument.as_ptr(), price, size) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Last() returned an unexpected value"),
    }
}

fn last_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let price: c_double = c_double::try_from(price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();
    let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

    let result: c_int = unsafe { LastPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("LastPlayback() returned an unexpected value"),
    }
}

fn market_data(instrument: &str, market_data_type: MarketDataType) -> f64 {
    let instrument: CString = CString::new(instrument).unwrap();
    let market_data_type: c_int = c_int::try_from(match market_data_type {
        Last => 0,
        Bid => 1,
        Ask => 2,
    }).unwrap();
    let result: c_double = unsafe { MarketData(instrument.as_ptr(), market_data_type) };
    let result: f64 = f64::try_from(result).unwrap();
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

fn order_status(order_id: &str) -> String {
    let order_id: CString = CString::new(order_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(OrderStatus(order_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    return result
}

fn realized_pnl(account: &str) -> f64 {
    let account: CString = CString::new(account).unwrap();
    let result: c_double = unsafe { RealizedPnL(account.as_ptr())};
    let result: f64 = f64::try_from(result).unwrap();
    return result
}

// check integer types for ports, unsigned?
fn setup(host: &str, port: i32) -> bool {
    let host: CString = CString::new(host).unwrap();
    let port: c_int = c_int::try_from(port).unwrap();

    let result: c_int = unsafe { SetUp(host.as_ptr(), port) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("Ask() returned an unexpected value"),
    }
}

fn get_stop_orders(strategy_id: &str) -> Vec<String> {
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(StopOrders(strategy_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}

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
    let result: c_int = unsafe { StrategyPosition(strategy_id.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    return result
}

fn subscribe_market_data(instrument: &str) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let result: c_int = unsafe { SubscribeMarketData(instrument.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}

fn get_target_orders(strategy_id: &str) -> Vec<String> {
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let result: &CStr = unsafe { CStr::from_ptr(Orders(strategy_id.as_ptr())) };
    let result: String = result.to_str().unwrap().to_string();
    let result: Vec<String> = separate(result);
    return result
}

fn tear_down() -> bool {
    let result: c_int = unsafe { TearDown() };    
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}

fn unsubscribe_market_data(instrument: &str) -> bool {
    let instrument: CString = CString::new(instrument).unwrap();
    let result: c_int = unsafe { UnsubscribeMarketData(instrument.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => return true,
        -1 => return false,
        _ => panic!("SubscribeMarketData() returned an unexpected value"),
    }
}
