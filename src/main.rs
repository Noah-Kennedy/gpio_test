extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    for i in 0..30 {
        let result = blink_led(i);
        match result {
            Some() => println!("Successfully blinked LED at {}!", i),
            None => println!("Failed to blink LED at {}!", i)
        }
    }
}

fn blink_led(led_pin: u64) -> sysfs_gpio::Result<()> {
    let my_led = Pin::new(led_pin); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    })
}