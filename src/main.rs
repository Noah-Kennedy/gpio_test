extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    for i in 0..30 {
        let result = blink_led(i);
        match result {
            Ok(_) => println!("Successfully blinked LED at {}!", i),
            Err(_) => println!("Failed to blink LED at {}!", i)
        }
    }
}

fn blink_led(led_pin: u64) -> sysfs_gpio::Result<()> {
    let my_led = Pin::new(led_pin); // number depends on chip, etc.
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Low).expect("Failed to set direction!");
        
        let mut result = my_led.set_value(0);
        if result.is_err() {
            return result;
        }
        
        sleep(Duration::from_millis(200));
        
        result = my_led.set_value(1);
        if result.is_err() {
            return result;
        }
        
        sleep(Duration::from_millis(200));
        Ok(())
    })
}