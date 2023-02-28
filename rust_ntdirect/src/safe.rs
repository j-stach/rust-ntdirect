
mod helpers;
mod errors;
mod commands;


#[allow(dead_code)] 
pub mod safe {

    use std::ffi::*;
    use std::result::Result; 
    use chrono::{DateTime, Utc};

    use crate::raw::*;
    use crate::safe::helpers::*;
    use crate::safe::helpers::helpful_types::*;
    use crate::safe::errors::{NTDirectError, NTDirectError::*};

    /// Sets the Ask price and size for the specified instrument?? Clarify what this means
    fn ask(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let result: c_int = unsafe { Ask(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("Ask".to_string())),
            _ => panic!("Ask() returned an unexpected value"),
        }
    }

    /// Same as Ask, but able to sync with an external playback?? Clarify what this means
    fn ask_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let timestamp: CString = CString::new(format_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { AskPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("AskPlayback".to_string())),
            _ => panic!("AskPlayback() returned an unexpected value"),
        }
    }

    /// Gets an account's average entry price for the specified instrument.
    fn avg_entry_price(instrument: &str, account: &str) -> f64 {
        let instrument: CString = CString::new(instrument).unwrap();
        let account: CString = CString::new(account).unwrap();
        let result: c_double = unsafe { AvgEntryPrice(instrument.as_ptr(), account.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    /// Gets the average fill price for an order.
    fn avg_fill_price(order_id: &str) -> f64 {
        let order_id: CString = CString::new(order_id).unwrap();
        let result: c_double = unsafe { AvgFillPrice(order_id.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    /// Sets the bid price and size for a specified instrument?? Clarify what this means
    fn bid(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();

        let result: c_int = unsafe { Bid(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("Bid".to_string())),
            _ => panic!("Bid() returned an unexpected value"),
        }
    }

    /// Same as Bid, but able to sync to an external playback?? Clarify what this means
    fn bid_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let timestamp: CString = CString::new(format_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { BidPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("BidPayback".to_string())),
            _ => panic!("BidPlayback() returned an unexpected value"),
        }
    }

    /// Gets the buying power for an account. (Not all brokerage technologies support this value.)
    fn buying_power(account: &str) -> f64 {
        let account: CString = CString::new(account).unwrap();
        let result: c_double = unsafe { BuyingPower(account.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result   
    }

    /// Gets the cash value for an account. (Not all brokerage technologies support this value.)
    fn cash_value(account: &str) -> f64 {
        let account: CString = CString::new(account).unwrap();
        let result: c_double = unsafe { CashValue(account.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    /* todo */
    // may need debugging. does it return an error or the value of the toggle?
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


    /// Attempts to initiate a connection to the NT8 platform
    fn connected(show_message: bool) -> Result<(), NTDirectError> {
        let show_message: c_int = c_int::try_from(match show_message {
            true => 1,
            false => 0,
        }).unwrap();
        let result: c_int = unsafe { Connected(show_message) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(ConnectionError("No connection established".to_string())),
            _ => panic!("Connection() returned an unexpected value"),
        }
    }

    /// Gets the number of contracts filled for the specified order.
    fn filled(order_id: &str) -> i32 {
        let order_id: CString = CString::new(order_id).unwrap();
        let result: c_int = unsafe { Filled(order_id.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        return result
    }

    /// Sets the Last price for an instrument?? Clarify what this means
    fn last(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();

        let result: c_int = unsafe { Last(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("Last".to_string())),
            _ => panic!("Last() returned an unexpected value"),
        }
    }

    /// Same as Last, but able to sync with an external playback?? Clarify what this means
    fn last_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let timestamp: CString = CString::new(format_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { LastPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(OrderError("LastPayback".to_string())),
            _ => panic!("LastPlayback() returned an unexpected value"),
        }
    }

    /// Gets the latest price by data type for an instrument. Must be subscribed to a data stream for that instrument.
    fn market_data(instrument: &str, market_data_type: MarketDataType) -> f64 {
        let instrument: CString = CString::new(instrument).unwrap();
        let market_data_type: c_int = c_int::try_from(match market_data_type {
            MarketDataType::Last => 0,
            MarketDataType::Bid => 1,
            MarketDataType::Ask => 2,
        }).unwrap();
        let result: c_double = unsafe { MarketData(instrument.as_ptr(), market_data_type) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    /* todo */
    // enum for position?
    fn market_position(instrument: &str, account: &str) -> i32 {
        let instrument: CString = CString::new(instrument).unwrap();
        let account: CString = CString::new(account).unwrap();
        let result: c_int = unsafe { MarketPosition(instrument.as_ptr(), account.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        return result
    }

    /// Creates a new unique order id value.
    fn new_order_id() -> String {
       let result: &CStr = unsafe { CStr::from_ptr(NewOrderId()) };
       let result: String = result.to_str().unwrap().to_string();
       return result
    }

    /// Gets all active orders for an account.
    fn orders(account: &str) -> Vec<String> {
        let account: CString = CString::new(account).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(Orders(account.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        let result: Vec<String> = separate(result);
        return result
    }

    /* todo */
    // custom type for order status: can it be an enum?
    fn order_status(order_id: &str) -> String {
        let order_id: CString = CString::new(order_id).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(OrderStatus(order_id.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        return result
    }

    /// Gets the realized profit/loss for an account.
    fn realized_pnl(account: &str) -> f64 {
        let account: CString = CString::new(account).unwrap();
        let result: c_double = unsafe { RealizedPnL(account.as_ptr())};
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    /* todo */
    // check integer types for ports, unsigned? TcpStream destructure?
    fn setup(host: &str, port: i32) -> Result<(), NTDirectError> {
        let host: CString = CString::new(host).unwrap();
        let port: c_int = c_int::try_from(port).unwrap();

        let result: c_int = unsafe { SetUp(host.as_ptr(), port) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(ConfigurationError("Setup".to_string())),
            _ => panic!("SetUp() returned an unexpected value"),
        }
    }

    /// Gets all stop loss orders for an active strategy.
    fn get_stop_orders(strategy_id: &str) -> Vec<String> {
        let strategy_id: CString = CString::new(strategy_id).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(StopOrders(strategy_id.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        let result: Vec<String> = separate(result);
        return result
    }

    /// Gets a list of all active strategies.
    fn strategies(account: &str) -> Vec<String> {
        let account: CString = CString::new(account).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(Strategies(account.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        let result: Vec<String> = separate(result);
        return result
    }
    
    /* todo */
    // enum like market position?
    fn strategy_position(strategy_id: &str) -> i32 {
        let strategy_id: CString = CString::new(strategy_id).unwrap();
        let result: c_int = unsafe { StrategyPosition(strategy_id.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        return result
    }

    /// Starts a new market data stream for a specified instrument.
    fn subscribe_market_data(instrument: &str) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let result: c_int = unsafe { SubscribeMarketData(instrument.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(ConnectionError("Unable to connect to market data stream".to_string())),
            _ => panic!("SubscribeMarketData() returned an unexpected value"),
        }
    }

    /// Gets the profit target orders for an active strategy.
    fn get_target_orders(strategy_id: &str) -> Vec<String> {
        let strategy_id: CString = CString::new(strategy_id).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(TargetOrders(strategy_id.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        let result: Vec<String> = separate(result);
        return result
    }

    /// Disconnects from the NT8 platform
    fn tear_down() -> Result<(), NTDirectError> {
        let result: c_int = unsafe { TearDown() };    
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(ConfigurationError("Teardown".to_string())),
            _ => panic!("TearDown() returned an unexpected value"),
        }
    }

    /// Stops the data stream for an instrument. Remember to call this on clean up or the connection may remain open.
    fn unsubscribe_market_data(instrument: &str) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let result: c_int = unsafe { UnsubscribeMarketData(instrument.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(ConnectionError("Failed to unsubscribe from market data stream".to_string())),
            _ => panic!("UnsubscribeMarketData() returned an unexpected value"),
        }
    }
}
