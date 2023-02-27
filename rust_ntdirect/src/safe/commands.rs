
    use std::ffi::*;
    use std::result::Result; 

    use crate::raw::*;
    use crate::safe::helpers::helpful_types::*;
    use crate::safe::errors::{NTDirectError, NTDirectError::*};


fn command(nt_command: NTCommand, account: Option<&str>, instrument: Option<&str>, action: Option<NTAction>, 
           size: Option<i32>, order_type: Option<OrderType>, limit_price: Option<f64>, stop_price: Option<f64>, 
           tif: Option<TIF>, oco: Option<&str>, order_id: Option<&str>, strategy: Option<&str>, strategy_id: Option<&str>) 
           -> Result<(), NTDirectError> {

    let command: CString = CString::new(nt_command.to_string()).unwrap();
    
    let account: *const c_char = string_opt_to_ptr(account.map(|a| CString::new(a).unwrap()));
    let instrument: *const c_char = string_opt_to_ptr(instrument.map(|i| CString::new(i).unwrap()));
    let action: *const c_char = string_opt_to_ptr(action.map(|x| CString::new(x.to_string()).unwrap()));
    let limit_price: *const c_double = double_opt_to_ptr(limit_price.map(|lp| c_double::try_from(lp).unwrap()));
    let stop_price: *const c_double = double_opt_to_ptr(stop_price.map(|sp| c_double::try_from(sp).unwrap()));
    let size: *const c_int = int_opt_to_ptr(size.map(|s| c_int::try_from(s).unwrap()));
    let order_type: *const c_char = string_opt_to_ptr(order_type.map(|t| CString::new(t.to_string()).unwrap()));
    let order_id: *const c_char = string_opt_to_ptr(order_id.map(|id| CString::new(id).unwrap()));
    let strategy_id: *const c_char = string_opt_to_ptr(strategy_id.map(|id| CString::new(id).unwrap()));
    let strategy: *const c_char = string_opt_to_ptr(strategy.map(|s| CString::new(s).unwrap()));
    let tif: *const c_char = string_opt_to_ptr(tif.map(|tif| CString::new(tif.to_string()).unwrap()));
    let oco: *const c_char = string_opt_to_ptr(oco.map(|oco| CString::new(oco).unwrap()));

    let result: c_int = unsafe { Command(command.as_ptr(), account, instrument, 
                                action, size, order_type, limit_price, stop_price, 
                                  tif, oco, order_id, strategy, strategy_id) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => Ok(()),
        -1 => Err(CommandError(nt_command.to_string())),
        _ => panic!("Command() returned an unexpected value"),
    }
}


fn cancel(order_id: &str, strategy_id: Option<&str>) {
    
}

fn cancel_all_orders() {}

fn change(order_id: &str, size: Option<i32>, limit_price: Option<f64>, stop_price: Option<f64>, strategy_id: Option<&str>) {

}

fn close_position(account: &str, instrument: &str) {

}

fn close_strategy(strategy_id: &str) {

}

fn flatten_everything() {}

fn place(account: &str, instrument: &str, action: NTAction, size: i32, order_type: OrderType, 
         limit_price: Option<f64>, stop_price: Option<f64>, tif: Option<TIF>, oco: Option<&str>, 
         order_id: Option<&str>, strategy: Option<&str>, strategy_id: Option<&str>) {

}

fn reverse_position(account: &str, instrument: &str, action: NTAction, size: i32, order_type: OrderType, 
         limit_price: Option<f64>, stop_price: Option<f64>, tif: Option<TIF>, oco: Option<&str>, 
         order_id: Option<&str>, strategy: Option<&str>, strategy_id: Option<&str>) {

}

