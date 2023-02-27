

use chrono::{DateTime, Utc};

#[allow(dead_code)]
pub(super) fn format_datetime(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y%m%d%H%M%S").to_string()
}

#[allow(dead_code)]
pub(super) fn separate(list: String) -> Vec<String> {
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


#[allow(dead_code)]
pub mod helpful_types {
    use std::{fmt::Display, ffi::*,};
    
    #[derive(Clone, Copy)]
    pub enum MarketDataType {
        Last,
        Ask,
        Bid,
    }

    pub enum NTCommand {
        Cancel,
        CancelAllOrders,
        Change,
        ClosePosition,
        CloseStragegy,
        FlattenEverything,
        Place,
        ReversePosition,
    }

    impl Display for NTCommand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NTCommand::Cancel => write!(f, "CANCEL"), 
                NTCommand::CancelAllOrders => write!(f, "CANCELALLORDERS"), 
                NTCommand::Change => write!(f, "CHANGE"), 
                NTCommand::ClosePosition => write!(f, "CLOSEPOSITION"), 
                NTCommand::CloseStragegy => write!(f, "CLOSESTRATEGY"), 
                NTCommand::FlattenEverything => write!(f, "FLATTENEVERYTHING"), 
                NTCommand::Place => write!(f, "PLACE"), 
                NTCommand::ReversePosition => write!(f, "REVERSEPOSITION"), 
            }
        }
    }

    pub enum NTAction { Buy, Sell, }
    impl Display for NTAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NTAction::Buy => write!(f, "BUY"),
                NTAction::Sell => write!(f, "SELL"),
            }
        }
    }

    pub enum OrderType {
        Market,
        Limit,
        StopMarket,
        StopLimit,
    }
    impl Display for OrderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                OrderType::Market => write!(f, "MARKET"),
                OrderType::Limit => write!(f, "LIMIT"),
                OrderType::StopMarket => write!(f, "STOPMARKET"),
                OrderType::StopLimit => write!(f, "STOPLIMIT"),
            }
        }
    }

    pub enum TIF { Day, GTC, }
    impl Display for TIF {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TIF::Day => write!(f, "DAY"),
                TIF::GTC => write!(f, "GTC"),
            }
        }
    }

    // Functions to handle Options for the FFI
    pub fn string_opt_to_ptr(s: Option<CString>) -> *const c_char {
        match s {
            Some(inner) => inner.as_ptr(),
            None => std::ptr::null(),
        }
    }
    pub fn double_opt_to_ptr(s: Option<f64>) -> *const c_double {
        match s {
            Some(inner) => &inner as *const c_double,
            None => std::ptr::null(),
        }
    }
    pub fn int_opt_to_ptr(s: Option<i32>) -> *const c_int {
        match s {
            Some(inner) => &inner as *const c_int,
            None => std::ptr::null(),
        }
    }
}