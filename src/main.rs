#![no_std]
#![no_main]

/// 
/// This programs transform a hardcoded morse code string into a the blinking of a LED. If a button is pressed, the program restarts the morse code. While the button is pressed, the LED switches rapidly between on and off
/// 

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

enum MorseCode {
    DOT,
    LINE,
    SPACE
}

impl Default for MorseCode {
    fn default() -> Self {
        MorseCode::SPACE // or any other variant you want as the default
    }
}


const DOT_DELAY: u32 = 500u32;
const LINE_DELAY: u32 = DOT_DELAY * 2;
const SPACE_DELAY: u32 = LINE_DELAY * 2;
const NOTHING_DELAY: u32 = DOT_DELAY;
const REPEAT_DELAY: u32 = NOTHING_DELAY * 2;
const RESTART_DELAY: u32 = 50u32;

const MORSE_CODE_STRING: &str = "..-. .-. . -.. . .-. .. -.-";


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Hi!");

    // Set GPIO7 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();
    let button = io.pins.gpio9.into_pull_up_input();

    led.set_low().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a loop.
    let mut delay = Delay::new(&clocks);

    let mut code_array: [MorseCode; MORSE_CODE_STRING.len()] = Default::default();
    for (idx, code_char) in MORSE_CODE_STRING.chars().enumerate() {
        code_array[idx] = match code_char {
            '.' => MorseCode::DOT,
            '-' => MorseCode::LINE,
            ' ' => MorseCode::SPACE,
            _ => unreachable!(),
        }
    }

    let mut idx = 0;
    loop {
        if idx >= code_array.len() {
            idx = 0;
            delay.delay_ms(REPEAT_DELAY); 
        }

        // assumes that the led is always set low (not on) at this point
        match code_array[idx]
        { 
            MorseCode::DOT =>  {
                led.set_high().unwrap();
                delay.delay_ms(DOT_DELAY);
            }
            MorseCode::LINE =>  {
                led.set_high().unwrap();
                delay.delay_ms(LINE_DELAY);
            }
            MorseCode::SPACE => {
                delay.delay_ms(SPACE_DELAY);
            }            
        }

        if ! matches!(code_array[idx], MorseCode::SPACE) {
            led.set_low().unwrap();
            delay.delay_ms(NOTHING_DELAY);
        }
        idx += 1;
        
        while button.is_low().unwrap() {
            led.toggle().unwrap();
            delay.delay_ms(RESTART_DELAY);
            led.toggle().unwrap();
            delay.delay_ms(RESTART_DELAY);

            idx = 0;
        } 
    }
}
