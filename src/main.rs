mod arduino;
use arduino::{Arduino, OUTPUT, INPUT, HIGH, LOW};
mod led;
use led::LED;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    
    let arduino = Arduino::new();

    
    let mut led = LED::new(10, Rc::clone(&arduino));

    
    led.turn_on();
    println!("LED is ON");

   
    if let Some(state) = led.read_state() {
        println!("LED state: {}", state);
    }

   

}
