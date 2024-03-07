#![no_std]
#![no_main]

use defmt_rtt as _;
use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
    i2c::I2C,
    interrupt,
    peripherals::{Interrupt, Peripherals},
    prelude::*,
    timer::TimerGroup,
    Delay, Rtc,
};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    defmt::info!("Hello world!");
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400u32.kHz(),
        &clocks,
    );

    interrupt::enable(Interrupt::I2C_EXT0, interrupt::Priority::Priority1).unwrap();

    use icm42670::accelerometer::Accelerometer;
    let mut imu = icm42670::Icm42670::new(i2c, icm42670::Address::Primary).unwrap();

    loop {
        let gyro_norm = imu.gyro_norm().unwrap();
        let accelerometer = imu.accel_norm().unwrap();
        defmt::info!(
            "ACCEL  =  X: {} Y: {} Z: {}\t\tGYRO  =  X: {} Y: {} Z: {}",
            accelerometer.x,
            accelerometer.y,
            accelerometer.z,
            gyro_norm.x,
            gyro_norm.y,
            gyro_norm.z
        );

        delay.delay_ms(500u32);
    }
}
