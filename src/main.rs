mod arduino;
use arduino::{OUTPUT, INPUT, HIGH,Arduino};



fn main() {
    let mut arduino = Arduino::new();

    // Set pin 10 as OUTPUT and write HIGH
    arduino.pinMode(10, OUTPUT);
    arduino.dig_write(10, HIGH);

    // Set pin 10 as INPUT and read from it
    arduino.pinMode(10, INPUT);
    if let Some(value) = arduino.dig_read(10) {
        println!("Value of pin 10 is {}", value);
    }

    // Set pin 11 as INPUT and read from it
    arduino.pinMode(11, INPUT);
    if let Some(value) = arduino.dig_read(11) {
        println!("Value of pin 11 is {}", value);
    }

    
    arduino.dely(1000);
}