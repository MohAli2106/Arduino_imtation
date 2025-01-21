use crate::arduino::{Arduino, OUTPUT, HIGH, LOW};
use std::rc::Rc;
use std::cell::RefCell;
pub struct LED {
    pin: u8, // The pin number the LED is connected to
    arduino: Rc<RefCell<Arduino>>, // Reference to the Arduino object
}

impl LED {
    // Constructor to create a new instance of the LED
    pub fn new(pin: u8, arduino: Rc<RefCell<Arduino>>) -> Self {
        LED { pin, arduino }
    }

    // Method to turn on the LED
    pub fn turn_on(&mut self) {
        self.arduino.borrow_mut().pinMode(self.pin, OUTPUT);
        self.arduino.borrow_mut().dig_write(self.pin, HIGH);
    }

    // Method to turn off the LED
    pub fn turn_off(&mut self) {
        self.arduino.borrow_mut().pinMode(self.pin, OUTPUT);
        self.arduino.borrow_mut().dig_write(self.pin, LOW);
    }

    // Method to read the state of the LED (on or off)
    pub fn read_state(&self) -> Option<u8> {
        self.arduino.borrow_mut().dig_read(self.pin)
    }
}