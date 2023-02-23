// use cxx::*;
use std::ffi::*;


// Should create type aliases for these in future, wait until it becomes confusing on the safe side

#[link(name = "NTDirect", /* path = "" */)]
extern "C" {
    /// Sets the ask price and size for a specified instrument. Return value indicates success or error.
    pub fn Ask(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    /// Similar to Ask, but can be sync'ed to an external playback. 
    pub fn AskPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char) -> c_int;

    /// Gets an account's average entry price for the specified instrument.
    pub fn AvgEntryPrice(instrument: *const c_char, account: *const c_char) -> c_double;

    /// Gets the average fill price for an order.
    pub fn AvgFillPrice(order_id: *const c_char) -> c_double;

    /// Sets the bid price and size for a specified instrument. Return value indicates success or error.
    pub fn Bid(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    /// Similar to Bid, but can be sync'ed to an external playback.
    pub fn BidPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char);

    /// Gets the buying power for the specified account. (Not all brokerage technologies support this value.)
    pub fn BuyingPower(account: *const c_char) -> c_double;

    /// Gets the cash value for the specified account. (Not all brokerage technologies support this value.)
    pub fn CashValue(account: *const c_char) -> c_double;

    /// Function for managing active orders, positions, and strategies. Refer to NT documentation "Commands and Valid Parameters".
    pub fn Command(command: *const c_char, account: *const c_char, instrument: *const c_char, action: *const c_char, quantity: c_int, order_type: *const c_char, limit_price: c_double, stop_price: c_double, tif: *const c_char, oco: *const c_char, strategy: *const c_char, strategy_id: *const c_char) -> c_int;

    /// Indicates if an order confirmation message will appear. For use with NT trading platform.
    pub fn ConfirmOrders(confirm: c_int) -> c_int;

    /// Return value indicates NT platform connection status, 0 for true, -1 for false.
    pub fn Connected(show_message: c_int) -> c_int;

    /// Gets the number of contracts filled for the specified order.
    pub fn Filled(order_id: *const c_char) -> c_int;

    /// Sets the last price and size for the specified instrument. Return value indicates success or error.
    pub fn Last(instrument: *const c_char, price: c_double, size: c_int) -> c_int;

    /// Similar to Last, but can be sync'ed to an external playback.
    pub fn LastPlayback(instrument: *const c_char, price: c_double, size: c_int, timestamp: *const c_char) -> c_int;

    /// Gets the most recent price for the specified instrument. Type is 0 for Last, 1 for Bid, 2 for Ask.
    pub fn MarketData(instrument: *const c_char, market_data_type: c_int) -> c_double;

    /// Gets an account's market position for an instrument.
    pub fn MarketPosition(instrument: *const c_char, account: *const c_char) -> c_int;

    /// Gets a new unique order id value.
    pub fn NewOrderId() -> *const c_char;

    /// Gets a string of active orders separated by "|".
    pub fn Orders(account: *const c_char) -> *const c_char;

    /// Gets the state of an order.
    pub fn OrderStatus(order_id: *const c_char) -> *const c_char;

    /// Gets the realized profit/loss for an account.
    pub fn RealizedPnL(account: *const c_char) -> c_double;

    /// Optional set up for host and port number.
    pub fn SetUp(host: *const c_char, port: c_int) -> c_int;

    /// Gets a string of Stop Loss orders separated by "|". Used for ATM strategies.
    pub fn StopOrders(strategy_id: *const c_char) -> *const c_char;

    /// Gets a string of active strategies separated by "|".
    pub fn Strategies(account: *const c_char) -> *const c_char;

    /// Gets the market position for a strategy that is running.
    pub fn StrategyPosition(strategy_id: *const c_char) -> c_int;

    /// Starts a market data stream for the specified instrument. Return value indicates success or error.
    pub fn SubscribeMarketData(instrument: *const c_char) -> c_int;

    /// Similar to StopOrders(), but for Profit Target orders.
    pub fn TargetOrders(strategy_id: *const c_char) -> *const c_char;

    /// Disconnects the DLL from the NT server.
    pub fn TearDown() -> c_int;

    /// Stops a data stream for the specified instrument. Return value indicates success or error.
    pub fn UnsubscribeMarketData(instrument: *const c_char) -> c_int; 

}