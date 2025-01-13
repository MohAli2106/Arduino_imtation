use std::thread;
use std::collections::HashMap;



pub const input:u8=0;
pub const output:u8=1;

pub const low:u8=0;
pub const high:u8=1;


pub struct Arduino{
    pins:HashMap<u8,u8>,
    dig_pins:HashMap<u8,u8>,
    anlg_pins:HashMap<u8,u8>,
}