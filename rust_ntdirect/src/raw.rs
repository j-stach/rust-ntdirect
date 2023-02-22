// use cxx::*;
use std::ffi::*;


// Should create type aliases for these in future, wait until it becomes confusing on the safe side

#[link(name = "NTDirect", /* path = "" */)]
extern "C" {
    pub fn Ask(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    pub fn AskPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char) -> c_int;

    pub fn AvgEntryPrice(instrument: *const c_char, account: *const c_char) -> c_double;

    pub fn AvgFillPrice(order_id: *const c_char) -> c_double;

    pub fn Bid(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    pub fn BidPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char);

    pub fn BuyingPower(account: *const c_char) -> c_double;

    pub fn CashValue(account: *const c_char) -> c_double;

    pub fn Command(command: *const c_char, account: *const c_char, instrument: *const c_char, action: *const c_char, 
                   quantity: c_int, order_type: *const c_char, limit_price: c_double, stop_price: c_double, 
                   tif: *const c_char, oco: *const c_char, strategy: *const c_char, strategy_id: *const c_char) -> c_int;

    pub fn ConfirmOrders(confirm: c_int) -> c_int;

    pub fn Connected(show_message: c_int) -> c_int;

    pub fn Filled(order_id: *const c_char) -> c_int;

    pub fn Last(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    pub fn LastPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char) -> c_int;

    pub fn MarketData(instrument: *const c_char, data_type: c_int) -> c_double;

    pub fn MarketPosition(instrument: *const c_char, account: *const c_char) -> c_int;

    pub fn NewOrderId() -> *const c_char;

    pub fn Orders(account: *const c_char) -> *const c_char;

    pub fn OrderStatus(order_id: *const c_char) -> *const c_char;

    pub fn RealizedPnL(account: *const c_char) -> c_double;

    pub fn SetUp(host: *const c_char, port: c_int) -> c_int;

    pub fn StopOrders(strategy_id: *const c_char) -> *const c_char;

    pub fn Strategies(account: *const c_char) -> *const c_char;

    pub fn StrategyPosition(strategy_id: *const c_char) -> c_int;

    pub fn SubscribeMarketData(instrument: *const c_char) -> c_int;

    pub fn TargetOrders(strategy_id: *const c_char) -> *const c_char;

    pub fn TearDown() -> c_int;

    pub fn UnsubscribeMarketData(instrument: *const c_char) -> c_int; 

}