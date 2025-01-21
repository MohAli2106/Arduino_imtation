mod arduino;
use arduino::{Arduino, OUTPUT, INPUT, HIGH, LOW};
mod led;
use led::LED;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Create the Arduino object
    let arduino = Arduino::new();

    // Create an LED connected to pin 10
    let mut led = LED::new(10, Rc::clone(&arduino));

    // Turn on the LED
    led.turn_on();
    println!("LED is ON");

    // Read the state of the LED
    if let Some(state) = led.read_state() {
        println!("LED state: {}", state);
    }

    // Turn off the LED
    led.turn_off();
    println!("LED is OFF");

    // Read the state of the LED
    if let Some(state) = led.read_state() {
        println!("LED state: {}", state);
    }
}