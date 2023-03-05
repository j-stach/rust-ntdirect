use crate::safe::safe::*;
//use crate::raw::*;


pub mod raw;
pub mod safe;

fn main() {
    let connected: bool = match connected(true) {
        Ok(()) => true,
        Err(_) => false,
    };
    println!("Connected? ...{}!", connected.to_string())
}