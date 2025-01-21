# Arduino Simulator in Rust

This project is a basic simulation of an Arduino microcontroller written in Rust. It mimics Arduino's core functionality, including digital and analog pin control.

## Features
- Simulates **digital pin modes** (INPUT, OUTPUT).
- Supports **digital read/write** (HIGH, LOW).
- Simulates **analog read/write** (0-1023 values).

## Usage

### Setting Up

1. **Clone the repository**:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Build the project**:
    ```sh
    cargo build
    ```

3. **Run the project**:
    ```sh
    cargo run
    ```

### Example Code

In the [main.rs](http://_vscodecontentref_/0) file, you can see an example of how to use the `Arduino` struct to interact with pins:

1. **Initialize the Arduino**:
    ```rust
    let mut arduino = Arduino::new();
    ```

2. **Set pin modes**:
    ```rust
    arduino.pinMode(10, OUTPUT);
    arduino.pinMode(11, INPUT);
    ```

3. **Write to a digital pin**:
    ```rust
    arduino.dig_write(10, HIGH);
    ```

4. **Read from a digital pin**:
    ```rust
    if let Some(value) = arduino.dig_read(10) {
        println!("Value of pin 10 is {}", value);
    }
    ```

5. **Introduce a delay**:
    ```rust
    arduino.dely(1000);
    ```

### Full Example

Here is a full example of how to use the `Arduino` struct in the [main.rs](http://_vscodecontentref_/1) file:

```rust
mod arduino;
use arduino::{OUTPUT, INPUT, HIGH, LOW, Arduino};
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