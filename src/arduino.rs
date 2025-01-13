
use std ::{collections::HashMap,time::Duration,thread}; //



pub const OUTPUT: u8 = 1;

pub const LOW: u8 = 0;
pub const HIGH: u8 = 1;


pub struct Arduino{
    pins:HashMap<u8,u8>,
    dig_pins:HashMap<u8,u8>,
    anlg_pins:HashMap<u8,u16>,
}

impl Arduino{
    pub fn new() -> Self {
        Arduino{
            pins:HashMap::new(),
            dig_pins:HashMap::new(),
            anlg_pins:HashMap::new(),

        }
    }
    pub fn pinMode(&mut self,pin:u8,mode:u8){
        self.pins.insert(pin, mode);
    }

    pub fn dig_read(&self,pin:u8)->Option<u8>{
        self.dig_pins.get(&pin).copied()
    }

    pub fn dig_write(&mut self,pin:u8,value:u8){
       if let Some(&mode)= self.pins.get(&pin){
            if mode==OUTPUT{
                self.dig_pins.insert(pin,value);
            }println!("Pin {} is not set as output",pin);
        }println!("Pin {} is not set",pin);
    }
    pub fn anlg_write(&mut self, pin:u8,value:u16){
        if let Some(&mode)=self.pins.get(&pin){
            if mode==OUTPUT{
                self.anlg_pins.insert(pin,value);
                    
            }println!("Pin {} is not set as output",pin);

        }println!("Pin {} isnot set",pin)

    }
    pub fn anlg_read(&self,pin:u8)->Option<u16>{
        self.anlg_pins.get(&pin).copied()
    }

    pub fn dely(ms:u64){
        thread::sleep(Duration::from_millis(ms));
    }

}