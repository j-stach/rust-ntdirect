
    use std::ffi::*;
    use std::result::Result; 

    use crate::raw::*;
    use crate::safe::helpers::helpful_types::*;
    use crate::safe::errors::{NTDirectError, NTDirectError::*};



fn command(nt_command: NTCommand, account: &str, instrument: &str, action: NTAction, size: i32, order_type: OrderType, limit_price: f64, stop_price: f64, 
           tif: TIF, oco: &str, order_id: &str, strategy: &str, strategy_id: &str) -> Result<(), NTDirectError> {

    let command: CString = CString::new(nt_command.to_string()).unwrap();
    let account: CString = CString::new(account).unwrap();
    let instrument: CString = CString::new(instrument).unwrap();
    let action: CString = CString::new(action.to_string()).unwrap();
    let limit_price: c_double = c_double::try_from(limit_price).unwrap();
    let stop_price: c_double = c_double::try_from(stop_price).unwrap();
    let size: c_int = c_int::try_from(size).unwrap();
    let order_type: CString = CString::new(order_type.to_string()).unwrap();
    let order_id: CString = CString::new(order_id).unwrap();
    let strategy_id: CString = CString::new(strategy_id).unwrap();
    let strategy: CString = CString::new(strategy).unwrap();
    let tif: CString = CString::new(tif.to_string()).unwrap();
    let oco: CString = CString::new(oco).unwrap();

    let result: c_int = unsafe { Command(command.as_ptr(), account.as_ptr(), instrument.as_ptr(), 
                                  action.as_ptr(), size, order_type.as_ptr(), limit_price, stop_price, 
                                  tif.as_ptr(), oco.as_ptr(), order_id.as_ptr(), strategy.as_ptr(), strategy_id.as_ptr()) };
    let result: i32 = i32::try_from(result).unwrap();
    match result {
        0 => Ok(()),
        -1 => Err(CommandError(nt_command.to_string())),
        _ => panic!("Command() returned an unexpected value"),
    }
}