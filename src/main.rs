#![no_std]
#![no_main]

use arduino_hal::Adc;
use arduino_hal::hal::wdt;
use panic_halt as _;
use motor_shield::{init_ams, MotorCommands};
use motor_shield::ShieldLayout;
use motor_shield::MotorShield;
use motor_shield::MotorPort;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    let a0read = a0.analog_read(&mut adc);

    let mut motor_shield = init_ams!(
        ShieldLayout {
            port1: MotorPort::TwoMotors,
            port2: MotorPort::Empty,
        },
        dp,
        pins
    );

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms4000).unwrap();

    let motor = motor_shield.motor(1).unwrap();
    motor.enable();
    motor.run(MotorCommands::FORWARD);

    loop {
        ufmt::uwriteln!(&mut serial, "A0 reading: {}", a0read).unwrap();
        motor.speed(255);
        arduino_hal::delay_ms(2000);
        motor.speed(0);
        watchdog.feed();
    }
}
