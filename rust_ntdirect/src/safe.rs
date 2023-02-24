
mod helpers;
mod errors;

// custom enums for market position, order type, tif, oco

// custom types/type aliases for order id, strategy id, etc?

#[allow(dead_code)] 
pub mod safe {

    use std::ffi::*;
    use std::fmt::Error;
    use std::result::Result; 
    use chrono::{DateTime, Utc};

    use crate::raw::*;
    use crate::safe::helpers::*;
    use crate::safe::errors::NTDirectError;

    // needs result with error type
    fn ask(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let result: c_int = unsafe { Ask(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("Ask".to_string())),
            _ => panic!("Ask() returned an unexpected value"),
        }
    }

    // needs result with error type
    fn ask_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { AskPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("AskPlayback".to_string())),
            _ => panic!("AskPlayback() returned an unexpected value"),
        }
    }

    fn avg_entry_price(instrument: &str, account: &str) -> f64 {
        let instrument: CString = CString::new(instrument).unwrap();
        let account: CString = CString::new(account).unwrap();
        let result: c_double = unsafe { AvgEntryPrice(instrument.as_ptr(), account.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    fn avg_fill_price(order_id: &str) -> f64 {
        let order_id: CString = CString::new(order_id).unwrap();
        let result: c_double = unsafe { AvgFillPrice(order_id.as_ptr()) };
        let result: f64 = f64::try_from(result).unwrap();
        return result
    }

    // needs result with error type
    fn bid(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();

        let result: c_int = unsafe { Bid(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("Bid".to_string())),
            _ => panic!("Bid() returned an unexpected value"),
        }
    }

    // needs result with error type
    fn bid_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { BidPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("BidPayback".to_string())),
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

    // needs result with error type
    // look into valid commands, may be able to make it easier
    fn command(nt_command: &str, account: &str, instrument: &str, action: &str, size: i32, order_type: &str, limit_price: f64, stop_price: f64, 
               tif: &str, oco: &str, order_id: &str, strategy: &str, strategy_id: &str) -> Result<(), NTDirectError> {

        let command: CString = CString::new(nt_command).unwrap();
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
                                      tif.as_ptr(), oco.as_ptr(), order_id.as_ptr(), strategy.as_ptr(), strategy_id.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::CommandError(nt_command.to_string())),
            _ => panic!("Command() returned an unexpected value"),
        }
    }

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


    // needs result with error type
    fn connected(show_message: bool) -> Result<(), NTDirectError> {
        let show_message: c_int = c_int::try_from(match show_message {
            true => 1,
            false => 0,
        }).unwrap();
        let result: c_int = unsafe { Connected(show_message) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::ConnectionError("No connection established".to_string())),
            _ => panic!("Connection() returned an unexpected value"),
        }
    }

    fn filled(order_id: &str) -> i32 {
        let order_id: CString = CString::new(order_id).unwrap();
        let result: c_int = unsafe { Filled(order_id.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        return result
    }

    // needs result with error type
    fn last(instrument: &str, price: f64, size: i32) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();

        let result: c_int = unsafe { Last(instrument.as_ptr(), price, size) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("Last".to_string())),
            _ => panic!("Last() returned an unexpected value"),
        }
    }

    // needs result with error type
    fn last_playback(instrument: &str, price: f64, size: i32, timestamp: DateTime<Utc>) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let price: c_double = c_double::try_from(price).unwrap();
        let size: c_int = c_int::try_from(size).unwrap();
        let timestamp: CString = CString::new(string_from_datetime(timestamp)).unwrap();

        let result: c_int = unsafe { LastPlayback(instrument.as_ptr(), price, size, timestamp.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::OrderError("LastPayback".to_string())),
            _ => panic!("LastPlayback() returned an unexpected value"),
        }
    }

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

    // enum for position?
    fn market_position(instrument: &str, account: &str) -> i32 {
        let instrument: CString = CString::new(instrument).unwrap();
        let account: CString = CString::new(account).unwrap();
        let result: c_int = unsafe { MarketPosition(instrument.as_ptr(), account.as_ptr()) };
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

    // needs result with error type
    // check integer types for ports, unsigned? TcpStream destructure?
    fn setup(host: &str, port: i32) -> Result<(), NTDirectError> {
        let host: CString = CString::new(host).unwrap();
        let port: c_int = c_int::try_from(port).unwrap();

        let result: c_int = unsafe { SetUp(host.as_ptr(), port) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::ConfigurationError("Setup".to_string())),
            _ => panic!("SetUp() returned an unexpected value"),
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

    // needs result with error type
    fn subscribe_market_data(instrument: &str) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let result: c_int = unsafe { SubscribeMarketData(instrument.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::ConnectionError("Unable to connect to market data stream".to_string())),
            _ => panic!("SubscribeMarketData() returned an unexpected value"),
        }
    }

    fn get_target_orders(strategy_id: &str) -> Vec<String> {
        let strategy_id: CString = CString::new(strategy_id).unwrap();
        let result: &CStr = unsafe { CStr::from_ptr(TargetOrders(strategy_id.as_ptr())) };
        let result: String = result.to_str().unwrap().to_string();
        let result: Vec<String> = separate(result);
        return result
    }

    // needs result with error type
    fn tear_down() -> Result<(), NTDirectError> {
        let result: c_int = unsafe { TearDown() };    
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::ConfigurationError("Teardown".to_string())),
            _ => panic!("TearDown() returned an unexpected value"),
        }
    }

    // needs result with error type
    fn unsubscribe_market_data(instrument: &str) -> Result<(), NTDirectError> {
        let instrument: CString = CString::new(instrument).unwrap();
        let result: c_int = unsafe { UnsubscribeMarketData(instrument.as_ptr()) };
        let result: i32 = i32::try_from(result).unwrap();
        match result {
            0 => Ok(()),
            -1 => Err(NTDirectError::ConnectionError("Failed to unsubscribe from market data stream".to_string())),
            _ => panic!("UnsubscribeMarketData() returned an unexpected value"),
        }
    }
}